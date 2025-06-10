use bevy::prelude::*;
use core_engine_macros::{composite_workflow, composite_workflow_return};
use tokio::task::JoinHandle;

use crate::{
    chunk::types::ChunkOwnerId,
    chunk_loader::components::ChunkLoaderComponent,
    workflow::{composite_workflow_context::ScopedCompositeWorkflowContext, functions::handle_composite_workflow_return_now},
};

pub(crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut composite_workflow_handle: Local<Option<JoinHandle<ScopedCompositeWorkflowContext>>>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
) {
    let loader_entity = trigger.entity();
    let (_, loader_transform, loader) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            unreachable!(
                "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at trigger-time",
                loader_entity
            );
        }
    };
    let loader_position = loader_transform.translation.truncate();
    let loader_radius = loader.radius;
    let handle_is_some = (*composite_workflow_handle).is_some();
    let handle_is_finished = match *composite_workflow_handle {
        Some(ref handle) => handle.is_finished(),
        None => false,
    };

    if handle_is_some && handle_is_finished {
        let handle = composite_workflow_handle.take().unwrap();
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
    }
    if handle_is_some && !handle_is_finished {
        return;
    }

    let owner_id = loader.chunk_owner_id().clone();
    let handle = composite_workflow!(
        move in owner_id: ChunkOwnerId,
        move in loader_position: Vec2,
        move in loader_radius: u32,
    {
        debug!("Removing chunk loader: {:?}", owner_id.entity());
        let output = workflow!(IO, ChunkLoader::OnRemoveChunkLoader, Input {
            chunk_owner_id: owner_id,
            chunk_loader_position: loader_position,
            chunk_loader_radius: loader_radius,
        });
        workflow!(I, ChunkLoader::UnloadChunks, Input {
            inputs: output.unload_chunk_inputs
        });
    });
    *composite_workflow_handle = Some(handle);
}
