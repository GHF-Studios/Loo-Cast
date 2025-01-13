use bevy::prelude::*;

use crate::chunk::{components::ChunkComponent, enums::{ChunkAction, ChunkActionPriority}, functions::world_pos_to_chunk, resources::{ChunkActionBuffer, ChunkManager}};

use super::components::ChunkLoaderComponent;

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

pub(in crate) fn load_chunk(
    chunk_manager: &ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_coord: (i32, i32),
    chunk_owner: Option<Entity>
) {
    let is_loaded = chunk_manager.loaded_chunks.contains(&chunk_coord);
    let is_owned = chunk_manager.owned_chunks.contains_key(&chunk_coord);
    let is_spawning = chunk_action_buffer.is_spawning(&chunk_coord);
    let is_despawning = chunk_action_buffer.is_despawning(&chunk_coord);
    let is_transfering_ownership = chunk_action_buffer.is_transfering_ownership(&chunk_coord);

    if !is_loaded {
        if !is_spawning && !is_despawning && !is_transfering_ownership { 
            chunk_action_buffer.0.insert(chunk_coord, ChunkAction::Spawn { coord: chunk_coord, owner: chunk_owner, priority: ChunkActionPriority::Realtime });
        }
    } else if !is_owned && !is_despawning && !is_transfering_ownership && chunk_owner.is_some() {
        chunk_action_buffer.0.insert(chunk_coord, ChunkAction::TransferOwnership { coord: chunk_coord, new_owner: chunk_owner.unwrap(), priority: ChunkActionPriority::Realtime });
    }
}

pub(in crate) fn unload_chunk(
    chunk_manager: &ChunkManager,
    chunk_action_buffer: &mut ChunkActionBuffer,
    chunk_query: &Query<&ChunkComponent>,
    chunk_loader_query: &Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_coord: (i32, i32),
) {
    let loaded = chunk_manager.is_loaded(&chunk_coord);
    let despawning = chunk_action_buffer.is_despawning(&chunk_coord);

    if loaded && !despawning {
        let chunk = match chunk_query
            .iter()
            .find(|chunk| chunk.coord == chunk_coord
        ) {
            Some(chunk) => chunk,
            None => {
                unreachable!("Failed to unload chunk '{:?}': it is already despawned according to the Chunk Query", chunk_coord);
            }
        };

        match chunk_loader_query
            .iter()
            .find(|(loader_entity, transform, loader)| {
                if chunk.owner.is_some_and(|chunk_owner| chunk_owner == *loader_entity) {
                    return false;
                }

                is_chunk_in_loader_range(
                    &chunk_coord, 
                    transform.translation.truncate(), 
                    loader.radius
                )
            }) {
            Some((new_owner, _, _)) => {
                chunk_action_buffer.0.insert(chunk_coord, ChunkAction::TransferOwnership { coord: chunk_coord, new_owner, priority: ChunkActionPriority::Realtime });
            },
            None => {
                chunk_action_buffer.0.insert(chunk_coord, ChunkAction::Despawn { coord: chunk_coord, priority: ChunkActionPriority::Realtime });
            }
        };
    }
}