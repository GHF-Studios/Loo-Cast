//! Space-Engineers-style voxel hand for every active voxel world.

use bevy::prelude::*;

use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    game::{
        GameSet,
        item::{ItemAction, ItemActionHint, ItemCatalog, ItemDefinition, ItemId, UseItemRequest},
    },
    spatial::{
        SpatialScale, UsfPosition, UsfPrimaryInteractionSlice, UsfRuntimeChartState, UsfScaleLayer,
        UsfSemanticFrame,
    },
    voxel::{
        VoxelBrush, VoxelEdit, VoxelEditingDisabled, VoxelFrameEdit, VoxelFrameSnapshot,
        VoxelMaterialId, VoxelQueryPosition, VoxelRayHit, VoxelScaleDomain, VoxelScaleRealization,
        VoxelSemanticAuthority,
    },
};

pub const VOXEL_HAND: ItemId = ItemId::new("voxel_hand");

const TOOL_RANGE: f32 = 64.0;
const BRUSH_RADIUS: f32 = 2.0;
// Search remains local to the active Scale Slice; unprojectable distant chunks
// are not candidate hits for this bounded hand tool.
const MAX_CHUNK_QUERY_OFFSET_NATIVE: f32 = 1_000_000.0;

pub struct VoxelHandItemPlugin;

impl Plugin for VoxelHandItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(Update, use_voxel_hand.in_set(GameSet::Action));
    }
}

fn register_item(mut catalog: ResMut<ItemCatalog>) {
    catalog.register(ItemDefinition {
        id: VOXEL_HAND,
        name: "Voxel Hand",
        description: "Remove/add voxels. Secondary rock; Shift glass; Ctrl nebula.",
        action_hints: vec![
            ItemActionHint::new(ItemAction::PRIMARY, "Remove voxels"),
            ItemActionHint::new(ItemAction::SECONDARY, "Add voxels"),
        ],
    });
}

fn use_voxel_hand(
    mut uses: MessageReader<UseItemRequest>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Res<UsfPrimaryInteractionSlice>,
    spatial_frame: Res<UsfRuntimeChartState>,
    mut worlds: ParamSet<(
        Query<
            (
                Entity,
                &VoxelScaleRealization,
                &UsfScaleLayer,
                Option<&UsfLogicalRealizationOf>,
            ),
            Without<VoxelEditingDisabled>,
        >,
        Query<
            (
                Entity,
                &mut VoxelScaleRealization,
                &UsfScaleLayer,
                Option<&UsfLogicalRealizationOf>,
            ),
            Without<VoxelEditingDisabled>,
        >,
    )>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    mut authorities: Query<(
        &UsfPosition,
        &UsfSemanticFrame,
        &mut VoxelSemanticAuthority,
        &VoxelScaleDomain,
    )>,
) {
    for request in uses.read() {
        if request.item != VOXEL_HAND
            || (request.action != ItemAction::PRIMARY && request.action != ItemAction::SECONDARY)
        {
            continue;
        }

        let hit = {
            let readable_worlds = worlds.p0();
            find_voxel_hand_hit(
                &readable_worlds,
                &authority_partitions,
                &spatial_frame,
                active.scale(),
                request.aim.origin,
                request.aim.direction,
            )
        };
        let Some(hit) = hit else {
            continue;
        };
        let Some(edit) = voxel_hand_edit(
            request.action,
            request.aim.direction,
            hit.position,
            &keyboard,
        ) else {
            continue;
        };
        let world_entity = hit.world;
        let authority_entity = hit.authority;

        if let Some(authority_entity) = authority_entity {
            let Ok((body_origin, body_frame, mut authority, domain)) =
                authorities.get_mut(authority_entity)
            else {
                error!(
                    ?authority_entity,
                    "voxel realization points at a missing semantic authority"
                );
                continue;
            };
            let body_origin = *body_origin;
            let body_frame = *body_frame;
            let domain = *domain;
            let edit_snapshot = VoxelFrameSnapshot::new(body_origin, body_frame, active.scale());
            let Ok(frame_edit) = VoxelFrameEdit::from_world(edit, edit_snapshot) else {
                error!(
                    ?authority_entity,
                    "voxel edit could not be expressed in semantic body-local coordinates"
                );
                continue;
            };
            authority.record_edit(frame_edit);
            drop(authority);

            let mut realization_worlds = worlds.p1();
            for (_, mut world, layer, realization) in &mut realization_worlds {
                let same_authority = realization
                    .and_then(|logical| authority_partitions.get(logical.0).ok())
                    .is_some_and(|partition| partition.0 == authority_entity);
                if !domain.editable(layer.scale()) || !same_authority {
                    continue;
                }

                let snapshot = VoxelFrameSnapshot::new(body_origin, body_frame, layer.scale());
                if let Err(error) = world.apply_authority_edit(frame_edit, snapshot) {
                    error!(
                        ?error,
                        ?authority_entity,
                        scale = %layer.scale(),
                        "shared voxel edit could not update editable realization caches"
                    );
                }
            }
            continue;
        }

        let mut writable_worlds = worlds.p1();
        let Ok((_, mut world, _, _)) = writable_worlds.get_mut(world_entity) else {
            continue;
        };
        if let Err(error) = world.record_inline_edit(edit) {
            error!(?error, "voxel edit scope could not be indexed canonically");
        }
    }
}

/// Query returns one semantic hit, preserving nearest-distance tie behavior.
/// The runtime chunk translations are disposable search data, not edit authority.
struct VoxelHandHit {
    world: Entity,
    authority: Option<Entity>,
    position: VoxelQueryPosition,
    distance: f32,
}

fn find_voxel_hand_hit(
    worlds: &Query<
        (
            Entity,
            &VoxelScaleRealization,
            &UsfScaleLayer,
            Option<&UsfLogicalRealizationOf>,
        ),
        Without<VoxelEditingDisabled>,
    >,
    authority_partitions: &Query<&UsfAuthorityPartitionOf>,
    spatial_frame: &UsfRuntimeChartState,
    active_scale: SpatialScale,
    origin: Vec3,
    direction: Vec3,
) -> Option<VoxelHandHit> {
    let mut nearest: Option<VoxelHandHit> = None;
    for (world_entity, world, layer, realization) in worlds {
        if layer.scale() != active_scale {
            continue;
        }
        let Ok(local_origin) = spatial_frame.origin().reexpressed_at(layer.scale()) else {
            continue;
        };
        for (address, chunk) in world.active_dense_materializations() {
            let Ok(chunk_translation) = address
                .query_origin()
                .usf()
                .relative_native_bounded(&local_origin, MAX_CHUNK_QUERY_OFFSET_NATIVE)
            else {
                continue;
            };
            let chunk_local_origin = origin - chunk_translation;
            let Some(VoxelRayHit { position, distance }) =
                chunk.raycast(chunk_local_origin, direction, TOOL_RANGE)
            else {
                continue;
            };
            let Ok(semantic_hit) = address.query_origin().translated(position) else {
                continue;
            };
            if nearest
                .as_ref()
                .is_none_or(|current| distance < current.distance)
            {
                let authority = realization
                    .and_then(|logical| authority_partitions.get(logical.0).ok())
                    .map(|partition| partition.0);
                nearest = Some(VoxelHandHit {
                    world: world_entity,
                    authority,
                    position: semantic_hit,
                    distance,
                });
            }
        }
    }
    nearest
}

fn voxel_hand_edit(
    action: ItemAction,
    aim_direction: Vec3,
    hit: VoxelQueryPosition,
    keyboard: &ButtonInput<KeyCode>,
) -> Option<VoxelEdit> {
    let direction = aim_direction.normalize_or_zero();
    let offset = if action == ItemAction::PRIMARY {
        direction * (BRUSH_RADIUS * 0.35)
    } else {
        -direction * (BRUSH_RADIUS * 0.35)
    };
    let center = hit.translated(offset).ok()?;
    let brush = VoxelBrush::sphere(center, BRUSH_RADIUS);
    if action == ItemAction::PRIMARY {
        return Some(VoxelEdit::Remove { brush });
    }
    let control = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let material = if control {
        VoxelMaterialId::NEBULA
    } else if shift {
        VoxelMaterialId::GLASS
    } else {
        VoxelMaterialId::ROCK
    };
    Some(VoxelEdit::Add { brush, material })
}
