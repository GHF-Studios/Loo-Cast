use bevy::prelude::*;
use core_engine_macros::{composite_workflow, composite_workflow_return};
use tokio::task::JoinHandle;

use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::handle_composite_workflow_return_now;

pub(crate) fn update_chunk_loader_system(
    mut composite_workflow_handle: Local<Option<JoinHandle<ScopedCompositeWorkflowContext>>>,
) {
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

            workflow!(I, ChunkLoader::LoadChunks, Input { inputs: load_chunk_inputs });
            workflow!(I, ChunkLoader::UnloadChunks, Input { inputs: unload_chunk_inputs });
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
