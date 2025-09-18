use bevy::prelude::*;
use core_api_macros::{composite_workflow, composite_workflow_return};

use crate::{chunk::types::ChunkOwnerId, chunk_loader::components::ChunkLoader, workflow::functions::handle_composite_workflow_return_later};

// TODO: MAJOR: This silently drops observed chunk loader removals if one is already in-progress composite-workflow-wise, so for now:
// Concurrent chunk loader removals are unsound!
#[tracing::instrument(skip_all)]
pub(crate) fn observe_on_remove_chunk_loader(trigger: Trigger<OnRemove, ChunkLoader>, chunk_loader_query: Query<(&Transform, &ChunkLoader)>) {
    let loader_entity = trigger.target();
    let (loader_transform, loader) = match chunk_loader_query.get(loader_entity) {
        Ok(value) => value,
        Err(_) => {
            unreachable!(
                "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                loader_entity
            );
        }
    };
    let loader_position = loader_transform.translation.truncate();
    let loader_radius = loader.radius;

    let owner_id = loader.chunk_owner_id().clone();
    let handle = composite_workflow!(
        OnRemoveChunkLoader,
        move in owner_id: ChunkOwnerId,
        move in loader_position: Vec2,
        move in loader_radius: u32,
    {
        warn!("Running composite workflow 'OnRemoveChunkLoader'");

        // let output = workflow!(IO, ChunkLoader::OnRemoveChunkLoader, Input {
        //     chunk_owner_id: owner_id.clone(),
        //     chunk_loader_position: loader_position,
        //     chunk_loader_radius: loader_radius,
        // });
        // // TODO: VERY VERY IMPORTANT: THIS IS TERRIBLE FUCKING SHIT!!!!
        // We already use ChunkLoader::UnloadChunks in the chunk_loader systems, and workflows cannot be used concurrently!
        // workflow!(I, ChunkLoader::UnloadChunks, Input {
        //     inputs: output.unload_chunk_inputs
        // });
        workflow!(I, ChunkLoader::OnRemovedChunkLoader, Input {
            chunk_owner_id: owner_id
        });
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();
        warn!("Finished composite workflow 'OnRemoveChunkLoader'");
    });
}
