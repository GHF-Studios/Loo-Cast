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
const REPEAT_INTERVAL_SECONDS: f32 = 0.05;

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
    time: Res<Time>,
    mut repeat_timer: Local<Option<Timer>>,
    mut uses: MessageReader<UsePlaygroundItem>,
    mut worlds: ParamSet<(
        Query<(Entity, &VoxelWorld)>,
        Query<&mut VoxelWorld>,
    )>,
    mut chunks: ParamSet<(Query<&VoxelChunk>, Query<&mut VoxelChunk>)>,
) {
    let timer = repeat_timer.get_or_insert_with(|| {
        Timer::from_seconds(REPEAT_INTERVAL_SECONDS, TimerMode::Repeating)
    });
    let repeat_ready = timer.tick(time.delta()).just_finished();

    for request in uses.read() {
        if request.item != VOXEL_HAND {
            continue;
        }

        let (remove, held) = match request.action {
            PlaygroundItemAction::PRIMARY => (true, false),
            PlaygroundItemAction::SECONDARY => (false, false),
            PlaygroundItemAction::PRIMARY_HELD => (true, true),
            PlaygroundItemAction::SECONDARY_HELD => (false, true),
            _ => continue,
        };

        if held && !repeat_ready {
            continue;
        }
        if !held {
            timer.reset();
        }

        let mut nearest: Option<(Entity, VoxelRayHit)> = None;
        {
            let worlds = worlds.p0();
            let chunks = chunks.p0();

            for (world_entity, world) in &worlds {
                for chunk_entity in world.chunk_entities() {
                    let Ok(chunk) = chunks.get(chunk_entity) else {
                        continue;
                    };
                    let Some(hit) = chunk.raycast(
                        request.aim.origin,
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
        let center = if remove {
            position + direction * (BRUSH_RADIUS * 0.35)
        } else {
            position - direction * (BRUSH_RADIUS * 0.35)
        };
        let brush = VoxelBrush::sphere(center, BRUSH_RADIUS);

        let edit = if remove {
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
