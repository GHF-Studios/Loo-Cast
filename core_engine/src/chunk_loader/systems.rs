use bevy::prelude::*;
use core_engine_macros::{composite_workflow, composite_workflow_return};
use std::collections::HashSet;
use tokio::task::JoinHandle;

use crate::chunk::enums::ChunkAction;
use crate::chunk::functions::calculate_chunks_in_radius;
use crate::chunk::resources::ChunkActionBuffer;
use crate::chunk_loader::components::ChunkLoaderComponent;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::handle_composite_workflow_return_now;

pub(crate) fn update_chunk_loader_system(
    mut composite_workflow_handle: Local<Option<JoinHandle<ScopedCompositeWorkflowContext>>>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    for (_, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;
        let loader_range = calculate_chunks_in_radius(position, radius).into_iter().collect::<HashSet<(i32, i32)>>();

        let mut invalid_actions = vec![];
        for (chunk_coord, action) in chunk_action_buffer.iter() {
            match action {
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

        let mut invalid_chunk_actions = Vec::new();
        for chunk_coord in invalid_actions {
            chunk_action_buffer.remove_action(&chunk_coord);
            invalid_chunk_actions.push((chunk_coord, chunk_loader.id));
        }
    }

    let handle_is_some = (*composite_workflow_handle).is_some();
    let handle_is_finished = match *composite_workflow_handle {
        Some(ref handle) => handle.is_finished(),
        None => false,
    };

    if !handle_is_some {
        let handle = composite_workflow!({
            let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);
            let load_chunk_inputs = categorize_chunks_output.load_chunk_inputs;
            let unload_chunk_inputs = categorize_chunks_output.unload_chunk_inputs;

            let chunk_texture_handles = workflow!(
                IO,
                ChunkLoader::GetChunkTextureHandles,
                Input {
                    inputs: load_chunk_inputs.clone().into_iter().map(|input| input.chunk_coord).collect::<Vec<_>>()
                }
            );
            workflow!(
                I,
                ChunkLoader::LoadChunks,
                Input {
                    inputs: load_chunk_inputs,
                    texture_handles: chunk_texture_handles
                }
            );
            workflow!(
                I,
                ChunkLoader::UnloadChunks,
                Input {
                    inputs: unload_chunk_inputs
                }
            );
        });

        *composite_workflow_handle = Some(handle);
    }
    if handle_is_some && !handle_is_finished {
        return;
    }

    if handle_is_some && handle_is_finished {
        let handle = composite_workflow_handle.take().unwrap();
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
    }
}
