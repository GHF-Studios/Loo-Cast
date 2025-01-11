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

    let potential_chunks_to_spawn: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
    
    for chunk_coord in potential_chunks_to_spawn {
        load_chunk(
            &chunk_manager, 
            &mut chunk_action_buffer, 
            *chunk_coord, 
            Some(loader_entity)
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
    let (loader_entity, _, _) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            panic!("Failed to add chunk loader {:?}: Chunk Loader Query did not include it", loader_entity);
        }
    };

    let potential_chunks_to_despawn: Vec<&(i32, i32)> = chunk_manager.owned_chunks
        .iter()
        .filter_map(|(chunk, &owner)| if owner == loader_entity { Some(chunk) } else { None })
        .collect();

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