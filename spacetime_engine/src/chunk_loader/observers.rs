use std::collections::HashSet;

use bevy::prelude::*;

use crate::chunk::components::ChunkComponent;
use crate::chunk::enums::ChunkWorkflow;
use crate::chunk::functions::*;
use crate::chunk::resources::{ChunkWorkflowBuffer, ChunkManager};

use super::components::ChunkLoaderComponent;
use super::functions::{load_chunk, unload_chunk};

// TODO: Re-Validate chunk workflows before the chunk unloading logic
pub(in crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnAdd, ChunkLoaderComponent>,
    chunk_loader_query: Query<(Entity, &Transform, &mut ChunkLoaderComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_workflow_buffer: ResMut<ChunkWorkflowBuffer>,
) {
    let loader_entity = trigger.entity();

    // Phase 1: Re-Validate chunk workflows


    // Phase 2: Perform chunk loading logic
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
            &mut chunk_workflow_buffer, 
            loader.id,
            *chunk_coord, 
            Some(loader_entity),
            chunk_loader_distance_squared,
            chunk_loader_radius_squared,
        );
    }
}

// TODO: Re-Validate chunk workflows before the chunk unloading logic
pub(in crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_workflow_buffer: ResMut<ChunkWorkflowBuffer>,
) {
    let loader_entity = trigger.entity();
    debug!("Handling removed chunk loader {}", loader_entity);
    let (_, loader_transform, loader) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            panic!(
                "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it",
                loader_entity
            );
        }
    };

    // Phase 1: Re-Validate chunk workflows

    let mut invalid_workflows = vec![];
    for (chunk_coord, workflow) in chunk_workflow_buffer.iter().filter(|(_, workflow)| workflow.get_requester_id() == loader.id) {
        match workflow {
            ChunkWorkflow::Spawn { .. } => {
                invalid_workflows.push(*chunk_coord);
            }
            ChunkWorkflow::Despawn { .. } => {}
            ChunkWorkflow::TransferOwnership { .. } => {}
        }
    }

    for chunk_coord in invalid_workflows {
        chunk_workflow_buffer.remove_workflow(&chunk_coord);
    }

    // Phase 2: Perform chunk unloading logic

    let position = loader_transform.translation.truncate();
    let radius = loader.radius;

    let chunks_to_despawn: Vec<&(i32, i32)> = chunk_manager
        .owned_chunks
        .iter()
        .filter_map(|(chunk, &owner)| {
            if owner == loader_entity { 
                chunk_workflow_buffer.remove_workflow(chunk);

                Some(chunk) 
            } else { 
                None 
            }
        })
        .collect();

    for &chunk_coord in chunks_to_despawn {
        let chunk_loader_distance_squared = calculate_chunk_distance_from_owner(
            &chunk_coord,
            &world_pos_to_chunk(position),
        );
        let chunk_loader_radius_squared = radius * radius;

        unload_chunk(
            &chunk_manager,
            &mut chunk_workflow_buffer,
            &chunk_query,
            &chunk_loader_query,
            loader.id,
            chunk_coord,
            chunk_loader_distance_squared,
            chunk_loader_radius_squared,
        );
    }
}
