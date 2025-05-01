use bevy::prelude::*;
use tokio::task::JoinHandle;
use spacetime_engine_macros::define_composite_workflow;

pub(crate) fn update_chunk_loader_system(mut composite_workflow_handle: Local<Option<JoinHandle<()>>>) {
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

    match *composite_workflow_handle {
        Some(ref handle) if handle.is_finished() => {
            *composite_workflow_handle = None;
        },
        Some(_) => todo!(),
        None => {
            *composite_workflow_handle = Some(crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
                .lock()
                .unwrap()
                .spawn_fallible(Box::pin(just_do_it())));
        }
    }

}
