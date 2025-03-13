use bevy::prelude::*;

use crate::chunk::{
    components::ChunkComponent,
    enums::{ChunkWorkflow, ChunkWorkflowPriority},
    functions::world_pos_to_chunk,
    resources::{ChunkManager, ChunkWorkflowBuffer},
};

use super::components::ChunkLoaderComponent;

fn calculate_spawn_priority(
    distance_squared: u32,
    radius_squared: u32,
    has_pending_despawn: bool,
) -> ChunkWorkflowPriority {
    let normalized_distance = distance_squared as f64 / radius_squared as f64;

    // Lower priority if a despawn is pending
    let adjustment = if has_pending_despawn { 0.5 } else { 1.0 };
    let priority_value = (i64::MAX as f64 * (1.0 - normalized_distance) * adjustment) as i64;

    ChunkWorkflowPriority::Deferred(priority_value)
}

fn calculate_despawn_priority(distance_squared: u32, radius_squared: u32) -> ChunkWorkflowPriority {
    let normalized_distance = distance_squared as f64 / radius_squared as f64;
    let priority_value = (normalized_distance * i64::MAX as f64) as i64;

    ChunkWorkflowPriority::Deferred(priority_value)
}

fn is_chunk_in_loader_range(
    chunk_coord: &(i32, i32),
    loader_position: Vec2,
    loader_radius: u32,
) -> bool {
    let (loader_chunk_x, loader_chunk_y) = world_pos_to_chunk(loader_position);

    let dx = chunk_coord.0 - loader_chunk_x;
    let dy = chunk_coord.1 - loader_chunk_y;
    let distance_squared = dx * dx + dy * dy;

    let radius_squared = (loader_radius as i32) * (loader_radius as i32);
    distance_squared <= radius_squared
}

pub(crate) fn load_chunk(
    chunk_manager: &ChunkManager,
    chunk_workflow_buffer: &mut ChunkWorkflowBuffer,
    requester_id: u32,
    chunk_coord: (i32, i32),
    chunk_owner: Option<Entity>,
    chunk_loader_distance_squared: u32,
    chunk_loader_radius_squared: u32,
) {
    let is_loaded = chunk_manager.loaded_chunks.contains(&chunk_coord);
    let is_owned = chunk_manager.owned_chunks.contains_key(&chunk_coord);
    let (is_spawning, is_despawning, is_transfering_ownership) =
        chunk_workflow_buffer.get_workflow_states(&chunk_coord);

    if !is_loaded {
        if !is_spawning && !is_despawning && !is_transfering_ownership {
            let has_pending_despawn = chunk_workflow_buffer.has_despawns();

            chunk_workflow_buffer.add_workflow(ChunkWorkflow::Spawn {
                requester_id,
                coord: chunk_coord,
                new_owner: chunk_owner,
                priority: calculate_spawn_priority(
                    chunk_loader_distance_squared,
                    chunk_loader_radius_squared,
                    has_pending_despawn,
                ),
            });
        }
    } else if !is_owned && !is_despawning && !is_transfering_ownership && chunk_owner.is_some() {
        chunk_workflow_buffer.add_workflow(ChunkWorkflow::TransferOwnership {
            requester_id,
            coord: chunk_coord,
            new_owner: chunk_owner.unwrap(),
            priority: ChunkWorkflowPriority::Realtime,
        });
    }
}

pub(crate) fn unload_chunk(
    chunk_manager: &ChunkManager,
    chunk_workflow_buffer: &mut ChunkWorkflowBuffer,
    chunk_query: &Query<(Entity, &ChunkComponent)>,
    chunk_loader_query: &Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    requester_id: u32,
    chunk_coord: (i32, i32),
    chunk_loader_distance_squared: u32,
    chunk_loader_radius_squared: u32,
) {
    let is_loaded = chunk_manager.is_loaded(&chunk_coord);
    let (is_spawning, is_despawning, is_transfering_ownership) =
        chunk_workflow_buffer.get_workflow_states(&chunk_coord);

    if is_loaded && !is_spawning && !is_despawning && !is_transfering_ownership {
        let chunk = match chunk_query
            .iter()
            .find(|(_, chunk)| chunk.coord == chunk_coord)
        {
            Some((_, chunk)) => chunk,
            None => {
                error!(
                    "Skipping unload for chunk '{:?}': it is already despawned",
                    chunk_coord
                );
                return;
            }
        };

        match chunk_loader_query
            .iter()
            .find(|(loader_entity, transform, loader)| {
                if chunk
                    .owner
                    .is_some_and(|chunk_owner| chunk_owner == *loader_entity)
                {
                    return false;
                }

                is_chunk_in_loader_range(
                    &chunk_coord,
                    transform.translation.truncate(),
                    loader.radius,
                )
            }) {
            Some((new_owner, _, _)) => {
                chunk_workflow_buffer.add_workflow(ChunkWorkflow::TransferOwnership {
                    requester_id,
                    coord: chunk_coord,
                    new_owner,
                    priority: ChunkWorkflowPriority::Realtime,
                });
            }
            None => {
                chunk_workflow_buffer.add_workflow(ChunkWorkflow::Despawn {
                    requester_id,
                    coord: chunk_coord,
                    priority: calculate_despawn_priority(
                        chunk_loader_distance_squared,
                        chunk_loader_radius_squared,
                    ),
                });
            }
        };
    }
}
