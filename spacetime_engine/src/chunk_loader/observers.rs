use bevy::prelude::*;
use spacetime_engine_macros::{composite_workflow, composite_workflow_return};

use crate::chunk::components::ChunkComponent;
use crate::chunk::resources::{ChunkManager, ChunkActionBuffer};
use crate::chunk_loader::components::ChunkLoaderComponent;
use crate::workflow::functions::handle_composite_workflow_return;

pub(crate) fn observe_on_add_chunk_loader(trigger: Trigger<OnAdd, ChunkLoaderComponent>) {
    let loader_entity = trigger.entity();

    let handle = composite_workflow!(loader_entity: Entity, JustDoIt {
        let output = workflow!(IO, ChunkLoader::OnAddChunkLoader, Input {
            chunk_loader_entity: loader_entity,
        });
        workflow!(I, ChunkLoader::LoadChunks, Input {
            inputs: output.load_chunk_inputs
        });
    });

    handle_composite_workflow_return(handle, || {
        composite_workflow_return!(loader_entity: Entity);
    });
}

pub(crate) fn observe_on_remove_chunk_loader(trigger: Trigger<OnRemove, ChunkLoaderComponent>) {
    let loader_entity = trigger.entity();

    let handle = composite_workflow!(loader_entity: Entity, JustDoIt {
        let output = workflow!(IO, ChunkLoader::OnRemoveChunkLoader, Input {
            chunk_loader_entity: loader_entity,
        });
        workflow!(I, ChunkLoader::UnloadChunks, Input {
            inputs: output.unload_chunk_inputs
        });
    });

    handle_composite_workflow_return(handle, || {
        composite_workflow_return!(loader_entity: Entity);
    });
}
