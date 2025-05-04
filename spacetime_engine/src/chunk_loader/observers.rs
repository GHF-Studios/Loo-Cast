use bevy::prelude::*;
use tokio::task::JoinHandle;
use spacetime_engine_macros::{composite_workflow, composite_workflow_return};

use crate::chunk_loader::components::ChunkLoaderComponent;

pub(crate) fn observe_on_add_chunk_loader(
    trigger: Trigger<OnAdd, ChunkLoaderComponent>,
    mut composite_workflow_handle: Local<Option<JoinHandle<()>>>,
) {
    let loader_entity = trigger.entity();
    let handle_is_some = (*composite_workflow_handle).is_some();
    let handle_is_finished = match *composite_workflow_handle {
        Some(ref handle) => handle.is_finished(),
        None => false,
    };
    
    if handle_is_some && handle_is_finished {
        *composite_workflow_handle = None;
        composite_workflow_return!(loader_entity: Entity);
    }
    if handle_is_some && !handle_is_finished {
        return;
    }

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

pub(crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoaderComponent>,
    mut composite_workflow_handle: Local<Option<JoinHandle<()>>>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
) {
    let loader_entity = trigger.entity();
    let (_, loader_transform, loader) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            panic!(
                "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at trigger-time",
                loader_entity
            );
        }
    };
    let loader_id = loader.id;
    let loader_position = loader_transform.translation.truncate();
    let loader_radius = loader.radius;
    let handle_is_some = (*composite_workflow_handle).is_some();
    let handle_is_finished = match *composite_workflow_handle {
        Some(ref handle) => handle.is_finished(),
        None => false,
    };
    
    if handle_is_some && handle_is_finished {
        *composite_workflow_handle = None;
        composite_workflow_return!(loader_entity: Entity, loader_id: u32, loader_position: Vec2, loader_radius: u32);
    }
    if handle_is_some && !handle_is_finished {
        return;
    }

    let handle = composite_workflow!(loader_entity: Entity, loader_id: u32, loader_position: Vec2, loader_radius: u32, JustDoIt {
        let output = workflow!(IO, ChunkLoader::OnRemoveChunkLoader, Input {
            chunk_loader_entity: loader_entity,
            chunk_loader_id: loader_id,
            chunk_loader_position: loader_position,
            chunk_loader_radius: loader_radius,
        });
        workflow!(I, ChunkLoader::UnloadChunks, Input {
            inputs: output.unload_chunk_inputs
        });
    });
    *composite_workflow_handle = Some(handle);
}
