//! Space-Engineers-style voxel hand for every active voxel world.

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    game::{
        GameSet,
        item::{ItemAction, ItemActionHint, ItemCatalog, ItemDefinition, ItemId, UseItem},
    },
    spatial::{UsfPrimaryInteractionSlice, UsfScaleLayer, UsfSpatialFrame},
    voxel::{VoxelAuthority, VoxelBrush, VoxelEdit, VoxelEditingDisabled, VoxelMaterialId, VoxelQueryPosition, VoxelRayHit, VoxelScaleDomain, VoxelWorld},
};

pub const VOXEL_HAND: ItemId = ItemId::new("voxel_hand");

const TOOL_RANGE: f32 = 64.0;
const BRUSH_RADIUS: f32 = 2.0;

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
    mut uses: MessageReader<UseItem>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Res<UsfPrimaryInteractionSlice>,
    spatial_frame: Res<UsfSpatialFrame>,
    mut worlds: ParamSet<(
        Query<
            (
                Entity,
                &VoxelWorld,
                &UsfScaleLayer,
                Option<&UsfManifestationOf>,
            ),
            Without<VoxelEditingDisabled>,
        >,
        Query<
            (
                Entity,
                &mut VoxelWorld,
                &UsfScaleLayer,
                Option<&UsfManifestationOf>,
            ),
            Without<VoxelEditingDisabled>,
        >,
    )>,
    mut authorities: Query<(&mut VoxelAuthority, &VoxelScaleDomain)>,
) {
    for request in uses.read() {
        if request.item != VOXEL_HAND
            || (request.action != ItemAction::PRIMARY
                && request.action != ItemAction::SECONDARY)
        {
            continue;
        }

        let mut nearest: Option<(Entity, Option<Entity>, VoxelQueryPosition, f32)> = None;
        {
            let worlds = worlds.p0();

            for (world_entity, world, layer, realization) in &worlds {
                if layer.scale() != active.scale() {
                    continue;
                }

                let Ok(local_origin) = spatial_frame.origin().reexpressed_at(layer.scale()) else {
                    continue;
                };

                for (address, chunk) in world.active_dense_materializations() {
                    let Ok(chunk_translation) = address
                        .query_origin()
                        .usf()
                        .relative_native_bounded(&local_origin, 1_000_000.0)
                    else {
                        continue;
                    };
                    let chunk_local_origin = request.aim.origin - chunk_translation;

                    let Some(VoxelRayHit { position, distance }) =
                        chunk.raycast(chunk_local_origin, request.aim.direction, TOOL_RANGE)
                    else {
                        continue;
                    };
                    let Ok(semantic_hit) = address.query_origin().translated(position) else {
                        continue;
                    };

                    if nearest.is_none_or(|(_, _, _, current)| distance < current) {
                        nearest = Some((
                            world_entity,
                            realization.map(|realization| realization.0),
                            semantic_hit,
                            distance,
                        ));
                    }
                }
            }
        }

        let Some((world_entity, authority_entity, hit, _)) = nearest else {
            continue;
        };

        let direction = request.aim.direction.normalize_or_zero();
        let offset = if request.action == ItemAction::PRIMARY {
            direction * (BRUSH_RADIUS * 0.35)
        } else {
            -direction * (BRUSH_RADIUS * 0.35)
        };
        let Ok(center) = hit.translated(offset) else {
            continue;
        };
        let brush = VoxelBrush::sphere(center, BRUSH_RADIUS);

        let edit = if request.action == ItemAction::PRIMARY {
            VoxelEdit::Remove { brush }
        } else {
            let control =
                keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
            let shift =
                keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
            let material = if control {
                VoxelMaterialId::NEBULA
            } else if shift {
                VoxelMaterialId::GLASS
            } else {
                VoxelMaterialId::ROCK
            };
            VoxelEdit::Add { brush, material }
        };

        if let Some(authority_entity) = authority_entity {
            let Ok((mut authority, domain)) = authorities.get_mut(authority_entity) else {
                error!(
                    ?authority_entity,
                    "voxel realization points at a missing semantic authority"
                );
                continue;
            };
            let domain = *domain;
            authority.record_edit(edit);
            drop(authority);

            let mut realization_worlds = worlds.p1();
            for (_, mut world, layer, realization) in &mut realization_worlds {
                if !domain.editable(layer.scale())
                    || realization
                        .is_none_or(|realization| realization.0 != authority_entity)
                {
                    continue;
                }

                if let Err(error) = world.apply_authority_edit(edit) {
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
        if let Err(error) = world.record_edit(edit) {
            error!(?error, "voxel edit scope could not be indexed canonically");
        }
    }
}

