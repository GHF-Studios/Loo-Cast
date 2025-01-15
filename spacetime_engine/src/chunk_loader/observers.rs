use std::collections::HashSet;

use bevy::prelude::*;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::*;
use crate::chunk::resources::{ChunkActionBuffer, ChunkManager};

use super::components::ChunkLoaderComponent;
use super::functions::{load_chunk, unload_chunk};

pub(in crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnAdd, ChunkLoaderComponent>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    let loader_entity = trigger.entity();
    let (loader_entity, loader_transform, loader) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            panic!("Failed to add chunk loader {:?}: Chunk Loader Query did not include it", loader_entity);
        }
    };
    
    let position = loader_transform.translation.truncate();
    let radius = loader.radius;

    let target_chunks = calculate_chunks_in_radius(position, radius)
        .into_iter()
        .collect::<HashSet<(i32, i32)>>();

    let current_chunks: HashSet<(i32, i32)> = chunk_manager.owned_chunks
        .iter()
        .filter_map(|(chunk, &owner)| if owner == loader_entity { Some(*chunk) } else { None })
        .collect();

    let chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
    
    for chunk_coord in chunks_to_spawn {
        let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
        let chunk_loader_radius_squared = radius * radius;

        load_chunk(
            &chunk_manager, 
            &mut chunk_action_buffer, 
            *chunk_coord, 
            Some(loader_entity),
            chunk_loader_distance_squared,
            chunk_loader_radius_squared,
        );
    }
}

pub(in crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    chunk_query: Query<&ChunkComponent>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    let loader_entity = trigger.entity();

    // Ensure we can fetch details about the removed loader
    let (_, loader_transform, loader) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            panic!(
                "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it",
                loader_entity
            );
        }
    };

    let position = loader_transform.translation.truncate();
    let radius = loader.radius;

    // Identify the chunks that belong to the removed loader
    let chunks_to_despawn: Vec<&(i32, i32)> = chunk_manager
        .owned_chunks
        .iter()
        .filter_map(|(chunk, &owner)| {
            if owner == loader_entity { 
                chunk_action_buffer.remove_action(chunk);

                Some(chunk) 
            } else { 
                None 
            }
        })
        .collect();

    // Process unloading of these chunks
    for &chunk_coord in chunks_to_despawn {
        let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(
            &chunk_coord,
            &world_pos_to_chunk(position),
        );
        let chunk_loader_radius_squared = radius * radius;

        unload_chunk(
            &chunk_manager,
            &mut chunk_action_buffer,
            &chunk_query,
            &chunk_loader_query,
            chunk_coord,
            chunk_loader_distance_squared,
            chunk_loader_radius_squared,
        );
    }
}
