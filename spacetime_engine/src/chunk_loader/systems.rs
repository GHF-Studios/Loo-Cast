use bevy::prelude::*;
use spacetime_engine_macros::define_composite_workflow;
use std::collections::HashSet;

use crate::chunk;
use crate::chunk::components::ChunkComponent;
use crate::chunk::enums::ChunkAction;
use crate::chunk::functions::{
    calculate_chunk_distance_from_owner, calculate_chunks_in_radius, world_pos_to_chunk,
};
use crate::chunk::resources::{ChunkManager, ChunkActionBuffer};

use super::components::ChunkLoaderComponent;
use super::workflows::*;

pub(crate) fn update_chunk_loader_system(
    chunk_query: Query<(Entity, &ChunkComponent)>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    // Phase 1: Re-Validate chunk actions
    for (_, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;
        let loader_range = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let mut invalid_actions = vec![];
        for (chunk_coord, workflow) in chunk_action_buffer.iter() {
            match workflow {
                ChunkAction::Spawn { .. } => {
                    if !loader_range.contains(chunk_coord) {
                        invalid_actions.push(*chunk_coord);
                    }
                }
                ChunkAction::Despawn { .. } => {
                    if loader_range.contains(chunk_coord) {
                        invalid_actions.push(*chunk_coord);
                    }
                }
                ChunkAction::TransferOwnership { .. } => {}
            }
        }

        for chunk_coord in invalid_actions {
            chunk_action_buffer.remove_action(&chunk_coord);

            warn!(
                "Removed invalid action for chunk {:?} in loader {:?}",
                chunk_coord, chunk_loader.id
            );
        }
    }

    // Phase 2: Perform chunk loading/unloading logic
    for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;

        let target_chunks = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let current_chunks: HashSet<(i32, i32)> = chunk_manager
            .owned_chunks
            .iter()
            .filter_map(|(chunk, &owner)| {
                if owner == loader_entity {
                    Some(*chunk)
                } else {
                    None
                }
            })
            .collect();

        let chunks_to_load: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
        let chunks_to_unload: Vec<&(i32, i32)> =
            current_chunks.difference(&target_chunks).collect();

        let mut load_chunk_inputs = Vec::new();
        for chunk_coord in chunks_to_load {
            let chunk_loader_distance_squared =
                calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
            let chunk_loader_radius_squared = radius * radius;

            load_chunk_inputs.push(crate::chunk_loader::workflows::chunk_loader::load_chunks::user_items::LoadChunkInput {
                requester_id: chunk_loader.id,
                chunk_coord: *chunk_coord,
                chunk_owner: Some(loader_entity),
                chunk_loader_distance_squared,
                chunk_loader_radius_squared,
            });
        }

        let mut unload_chunk_inputs = Vec::new();
        for chunk_coord in chunks_to_unload {
            let chunk_loader_distance_squared =
                calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
            let chunk_loader_radius_squared = radius * radius;

            unload_chunk_inputs.push(crate::chunk_loader::workflows::chunk_loader::unload_chunks::user_items::UnloadChunkInput {
                requester_id: chunk_loader.id,
                chunk_coord: *chunk_coord,
                chunk_loader_distance_squared,
                chunk_loader_radius_squared,
            });
        }

        define_composite_workflow!(JustDoIt {
            workflow!(E, ChunkLoader::ValidateChunkActions);
            let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);
            workflow!(I, ChunkLoader::LoadChunks, Input {
                inputs: categorize_chunks_output.load_chunk_inputs
            });
            workflow!(I, ChunkLoader::UnloadChunks, Input {
                inputs: categorize_chunks_output.unload_chunk_inputs
            });

            Ok(())
        });

        crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
            .lock()
            .unwrap()
            .spawn_fallible(Box::pin(just_do_it()));
    }
}
