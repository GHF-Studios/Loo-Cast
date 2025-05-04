use bevy::prelude::*;
use tokio::task::JoinHandle;
use spacetime_engine_macros::{composite_workflow, composite_workflow_return};

use crate::chunk_loader::components::ChunkLoaderComponent;

pub(crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnAdd, ChunkLoaderComponent>,
    mut composite_workflow_handle: Local<Option<JoinHandle<()>>>,
) {
    let loader_entity = trigger.entity();

    if (*composite_workflow_handle).is_none() {
        let handle = composite_workflow!(loader_entity: Entity, JustDoIt {
            let output = workflow!(IO, ChunkLoader::OnAddChunkLoader, Input {
                chunk_loader_entity: loader_entity,
            });
            workflow!(I, ChunkLoader::LoadChunks, Input {
                inputs: output.load_chunk_inputs
            });
        });

        *composite_workflow_handle = Some(handle);
    }

    if let Some(ref handle) = *composite_workflow_handle {
        if handle.is_finished() {
            *composite_workflow_handle = None;
            composite_workflow_return!();
        }
    }
}

pub(crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut composite_workflow_handle: Local<Option<JoinHandle<()>>>,
) {
    let loader_entity = trigger.entity();

    if (*composite_workflow_handle).is_none() {
        let handle = composite_workflow!(loader_entity: Entity, JustDoIt {
            let output = workflow!(IO, ChunkLoader::OnRemoveChunkLoader, Input {
                chunk_loader_entity: loader_entity,
            });
            workflow!(I, ChunkLoader::UnloadChunks, Input {
                inputs: output.unload_chunk_inputs
            });
        });

        *composite_workflow_handle = Some(handle);
    }

    if let Some(ref handle) = *composite_workflow_handle {
        if handle.is_finished() {
            *composite_workflow_handle = None;
            composite_workflow_return!();
        }
    }
}
