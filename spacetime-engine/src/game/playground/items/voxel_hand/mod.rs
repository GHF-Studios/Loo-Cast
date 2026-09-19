//! Space-Engineers-style voxel hand for every active voxel world.

use bevy::prelude::*;

use crate::{
    game::{
        GameSet,
        item::{ItemAction, ItemCatalog, ItemDefinition, ItemId, UseItem},
    },
    spatial::{UsfScaleLayer, UsfScaleLayerFrames},
    voxel::{VoxelBrush, VoxelEdit, VoxelMaterialId, VoxelQueryPosition, VoxelRayHit, VoxelWorld},
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
        description: "Remove or add smooth volumetric matter.",
    });
}

fn use_voxel_hand(
    mut uses: MessageReader<UseItem>,
    frames: Res<UsfScaleLayerFrames>,
    mut worlds: ParamSet<(
        Query<(Entity, &VoxelWorld, &UsfScaleLayer)>,
        Query<&mut VoxelWorld>,
    )>,
) {
    for request in uses.read() {
        if request.item != VOXEL_HAND
            || (request.action != ItemAction::PRIMARY
                && request.action != ItemAction::SECONDARY)
        {
            continue;
        }

        let mut nearest: Option<(Entity, VoxelQueryPosition, f32)> = None;
        {
            let worlds = worlds.p0();

            for (world_entity, world, layer) in &worlds {
                let world_origin = VoxelQueryPosition::new(*world.origin());

                for (address, chunk) in world.active_dense_materializations() {
                    let Ok(relative) = address
                        .query_origin()
                        .relative_to(world_origin, 1_000_000.0)
                    else {
                        continue;
                    };
                    let absolute = bevy::math::DVec3::new(
                        relative.x as f64,
                        relative.y as f64,
                        relative.z as f64,
                    );
                    let chunk_translation = frames.runtime_from_absolute(layer.scale(), absolute);
                    let chunk_local_origin = request.aim.origin - chunk_translation;

                    let Some(VoxelRayHit { position, distance }) =
                        chunk.raycast(chunk_local_origin, request.aim.direction, TOOL_RANGE)
                    else {
                        continue;
                    };
                    let Ok(semantic_hit) = address.query_origin().translated(position) else {
                        continue;
                    };

                    if nearest.is_none_or(|(_, _, current)| distance < current) {
                        nearest = Some((world_entity, semantic_hit, distance));
                    }
                }
            }
        }

        let Some((world_entity, hit, _)) = nearest else {
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
            VoxelEdit::Add {
                brush,
                material: VoxelMaterialId::ROCK,
            }
        };

        let mut worlds = worlds.p1();
        let Ok(mut world) = worlds.get_mut(world_entity) else {
            continue;
        };
        if let Err(error) = world.record_edit(edit) {
            error!(?error, "voxel edit scope could not be indexed canonically");
        }
    }
}
