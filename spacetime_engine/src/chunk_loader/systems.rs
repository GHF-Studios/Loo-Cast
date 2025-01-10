use bevy::prelude::*;
use std::collections::HashSet;

use crate::chunk::components::ChunkComponent;
use crate::chunk::functions::calculate_chunks_in_radius;
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

        let potential_chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let potential_chunks_to_despawn: Vec<&(i32, i32)> = current_chunks.difference(&target_chunks).collect();

        for chunk_coord in potential_chunks_to_spawn {
            load_chunk(
                &chunk_manager, 
                &mut chunk_action_buffer, 
                *chunk_coord, 
                Some(loader_entity)
            );
        }

        for chunk_coord in potential_chunks_to_despawn {
            unload_chunk(
                &chunk_manager, 
                &mut chunk_action_buffer,
                &chunk_query,
                &chunk_loader_query,
                *chunk_coord
            );
        }
    }
}
