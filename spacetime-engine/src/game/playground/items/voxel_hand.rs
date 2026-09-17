//! Space-Engineers-style voxel hand for every active voxel world.

use bevy::prelude::*;

use crate::{
    game::GameSet,
    voxel::{
        VoxelBrush, VoxelChunk, VoxelEdit, VoxelMaterialId, VoxelMaterializationChunkAddress,
        VoxelQueryPosition, VoxelRayHit, VoxelWorld,
    },
};

use super::super::{
    PlaygroundCatalog, PlaygroundItem, PlaygroundItemAction, PlaygroundItemId, UsePlaygroundItem,
};

pub const VOXEL_HAND: PlaygroundItemId = PlaygroundItemId::new("voxel_hand");

const TOOL_RANGE: f32 = 64.0;
const BRUSH_RADIUS: f32 = 2.0;

pub struct VoxelHandItemPlugin;

impl Plugin for VoxelHandItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(Update, use_voxel_hand.in_set(GameSet::Action));
    }
}

fn register_item(mut catalog: ResMut<PlaygroundCatalog>) {
    catalog.register(PlaygroundItem {
        id: VOXEL_HAND,
        name: "Voxel Hand",
        description: "Remove or add smooth volumetric matter.",
    });
}

fn use_voxel_hand(
    mut uses: MessageReader<UsePlaygroundItem>,
    mut worlds: ParamSet<(Query<(Entity, &VoxelWorld)>, Query<&mut VoxelWorld>)>,
    mut chunks: ParamSet<(
        Query<(
            &VoxelChunk,
            &Transform,
            &VoxelMaterializationChunkAddress,
        )>,
        Query<(&mut VoxelChunk, &VoxelMaterializationChunkAddress)>,
    )>,
) {
    for request in uses.read() {
        if request.item != VOXEL_HAND
            || (request.action != PlaygroundItemAction::PRIMARY
                && request.action != PlaygroundItemAction::SECONDARY)
        {
            continue;
        }

        let mut nearest: Option<(Entity, VoxelQueryPosition, f32)> = None;
        {
            let worlds = worlds.p0();
            let chunks = chunks.p0();

            for (world_entity, world) in &worlds {
                for chunk_entity in world.chunk_entities() {
                    let Ok((chunk, transform, address)) = chunks.get(chunk_entity) else {
                        continue;
                    };

                    // Dense chunk queries are strictly local to the projected
                    // materialization entity. Convert only the final local hit
                    // back into canonical USF space.
                    let chunk_local_origin = request.aim.origin - transform.translation;
                    let Some(VoxelRayHit { position, distance }) = chunk.raycast(
                        chunk_local_origin,
                        request.aim.direction,
                        TOOL_RANGE,
                    ) else {
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
        let offset = if request.action == PlaygroundItemAction::PRIMARY {
            direction * (BRUSH_RADIUS * 0.35)
        } else {
            -direction * (BRUSH_RADIUS * 0.35)
        };
        let Ok(center) = hit.translated(offset) else {
            continue;
        };
        let brush = VoxelBrush::sphere(center, BRUSH_RADIUS);

        let edit = if request.action == PlaygroundItemAction::PRIMARY {
            VoxelEdit::Remove { brush }
        } else {
            VoxelEdit::Add {
                brush,
                material: VoxelMaterialId::ROCK,
            }
        };

        // Record the canonical edit independently from dense caches, then patch
        // only currently materialized representations that intersect its scope.
        let affected = {
            let mut worlds = worlds.p1();
            let Ok(mut world) = worlds.get_mut(world_entity) else {
                continue;
            };
            let Ok(affected) = world.chunks_intersecting(edit.influence_bounds()) else {
                continue;
            };
            if let Err(error) = world.record_edit(edit) {
                error!(?error, "voxel edit scope could not be indexed canonically");
                continue;
            }
            affected
        };

        let mut chunks = chunks.p1();
        for entity in affected {
            if let Ok((mut chunk, address)) = chunks.get_mut(entity) {
                chunk.apply_edit(*address, edit);
            }
        }
    }
}
