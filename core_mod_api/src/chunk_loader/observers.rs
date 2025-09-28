use bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};

use crate::{chunk::types::ChunkOwnerId, chunk_loader::components::ChunkLoader, workflow::functions::handle_composite_workflow_return_later};

use super::types::RemovedChunkLoaderObservation;
use super::resources::RemovedChunkLoaderObservationQueue;

#[tracing::instrument(skip_all)]
pub(crate) fn observe_on_remove_chunk_loader(
    trigger: Trigger<OnRemove, ChunkLoader>,
    mut queue: ResMut<RemovedChunkLoaderObservationQueue>,
) {
    let loader_entity = trigger.target();
    queue.0.insert(RemovedChunkLoaderObservation { entity: loader_entity });
}

// TODO: MAJOR: This silently drops observed chunk loader removals if one is already in-progress composite-workflow-wise, so for now:
// Concurrent chunk loader removals are unsound!
#[tracing::instrument(skip_all)]
pub(crate) fn on_remove_chunk_loader_observation_queue_processing_system(
    chunk_loader_query: Query<&'static ChunkLoader>,
    mut queue: ResMut<RemovedChunkLoaderObservationQueue>,
) {
    let mut removed_owner_id = None;

    for RemovedChunkLoaderObservation { entity: loader_entity } in std::mem::take(&mut queue.0).into_iter() {
        let loader = match chunk_loader_query.get(loader_entity) {
            Ok(value) => value,
            Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
        };

        removed_owner_id = Some(loader.chunk_owner_id().clone());
    }

    if removed_owner_id.is_none() {
        return;
    }

    let handle = composite_workflow!(
        OnRemoveChunkLoader,
        //move in loader_position: Vec2,
        //move in loader_radius: u32,
        move in removed_owner_id: Option<ChunkOwnerId>,
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
            inner: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input { chunk_owner_id: removed_owner_id },
        });
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();
        warn!("Finished composite workflow 'OnRemoveChunkLoader'");
    });
}
