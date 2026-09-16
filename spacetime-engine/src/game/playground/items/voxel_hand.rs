//! Space-Engineers-style voxel hand for every active voxel world.

use bevy::prelude::*;

use crate::{
    game::GameSet,
    voxel::{VoxelBrush, VoxelChunk, VoxelEdit, VoxelMaterialId, VoxelRayHit, VoxelWorld},
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
    mut worlds: ParamSet<(
        Query<(Entity, &VoxelWorld, &Transform)>,
        Query<&mut VoxelWorld>,
    )>,
    mut chunks: ParamSet<(Query<&VoxelChunk>, Query<&mut VoxelChunk>)>,
) {
    for request in uses.read() {
        if request.item != VOXEL_HAND
            || (request.action != PlaygroundItemAction::PRIMARY
                && request.action != PlaygroundItemAction::SECONDARY)
        {
            continue;
        }

        let mut nearest: Option<(Entity, VoxelRayHit)> = None;
        {
            let worlds = worlds.p0();
            let chunks = chunks.p0();

            for (world_entity, world, world_transform) in &worlds {
                for chunk_entity in world.chunk_entities() {
                    let Ok(chunk) = chunks.get(chunk_entity) else {
                        continue;
                    };
                    // M7 keeps current voxel authority in VoxelWorld-local space.
                    // Convert the runtime ray through the world-root translation so
                    // edits and queries remain stable when the local origin rebases.
                    let world_local_origin = request.aim.origin - world_transform.translation;
                    let Some(hit) = chunk.raycast(
                        world_local_origin,
                        request.aim.direction,
                        TOOL_RANGE,
                    ) else {
                        continue;
                    };

                    if nearest
                        .is_none_or(|(_, current)| hit.distance < current.distance)
                    {
                        nearest = Some((world_entity, hit));
                    }
                }
            }
        }

        let Some((world_entity, VoxelRayHit { position, .. })) = nearest else {
            continue;
        };

        let direction = request.aim.direction.normalize_or_zero();
        let center = if request.action == PlaygroundItemAction::PRIMARY {
            position + direction * (BRUSH_RADIUS * 0.35)
        } else {
            position - direction * (BRUSH_RADIUS * 0.35)
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

        // First record the semantic edit independently of any currently loaded
        // dense chunk. Then update intersecting caches immediately.
        let affected = {
            let mut worlds = worlds.p1();
            let Ok(mut world) = worlds.get_mut(world_entity) else {
                continue;
            };
            world.record_edit(edit);
            world.chunks_intersecting(edit.influence_bounds())
        };

        let mut chunks = chunks.p1();
        for entity in affected {
            if let Ok(mut chunk) = chunks.get_mut(entity) {
                chunk.apply_edit(edit);
            }
        }
    }
}
