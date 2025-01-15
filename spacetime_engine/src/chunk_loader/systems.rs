use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::{calculate_chunk_distance_from_owner, calculate_chunks_in_radius, world_pos_to_chunk};
use crate::chunk::resources::{ChunkActionBuffer, ChunkManager};

use super::components::ChunkLoaderComponent;
use super::functions::{load_chunk, unload_chunk};

pub(in crate) fn update_chunk_loader_system(
    chunk_query: Query<&ChunkComponent>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let current_chunks: HashSet<(i32, i32)> = chunk_manager.owned_chunks
            .iter()
            .filter_map(|(chunk, &owner)| if owner == loader_entity { Some(*chunk) } else { None })
            .collect();

        let chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let chunks_to_despawn: Vec<&(i32, i32)> = current_chunks.difference(&target_chunks).collect();

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

        for chunk_coord in chunks_to_despawn {
            let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
            let chunk_loader_radius_squared = radius * radius;

            unload_chunk(
                &chunk_manager, 
                &mut chunk_action_buffer,
                &chunk_query,
                &chunk_loader_query,
                *chunk_coord,
                chunk_loader_distance_squared,
                chunk_loader_radius_squared,
            );
        }
    }
}
