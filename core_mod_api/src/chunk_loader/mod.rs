pub mod components;
pub mod observers;
pub mod resources;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;
use components::ChunkLoader;
use observers::{observe_on_remove_chunk_loader, on_remove_chunk_loader_observation_queue_processing_system};
use resources::{RemovedChunkLoaders, RemovedChunkLoaderObservationQueue};
use systems::{post_update_chunk_loader_system, update_chunk_loader_system};
use types::{RemovedChunkLoader, RemovedChunkLoaderObservation};

use crate::{
    core::run_conditions::run_after_startup_finished,
    time::run_conditions::run_if_not_paused,
    utils::{
        components::{DropHook, InitHook},
        functions::{cleanup_drop_hooks_system, cleanup_init_hooks_system, observe_on_remove_drop_hook, observe_on_remove_init_hook},
    },
};

pub(crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(observe_on_remove_chunk_loader)
            .add_observer(observe_on_remove_init_hook::<ChunkLoader>)
            .add_observer(observe_on_remove_drop_hook::<ChunkLoader>)

            .add_systems(
                PreUpdate,
                cleanup_drop_hooks_system::<ChunkLoader>.run_if(run_after_startup_finished.and(run_if_not_paused))
            )
            .add_systems(Update, (
                update_chunk_loader_system, 
                on_remove_chunk_loader_observation_queue_processing_system,
            ).run_if(run_after_startup_finished.and(run_if_not_paused)))
            .add_systems(
                PostUpdate,
                (cleanup_init_hooks_system::<ChunkLoader>, post_update_chunk_loader_system).run_if(run_after_startup_finished.and(run_if_not_paused))
            )
            
            .insert_resource(RemovedChunkLoaderObservationQueue::default())
            .insert_resource(RemovedChunkLoaders::default())

            .register_type::<ChunkLoader>()
            .register_type::<RemovedChunkLoader>()
            .register_type::<RemovedChunkLoaders>()
            .register_type::<InitHook<ChunkLoader>>()
            .register_type::<DropHook<ChunkLoader>>()
            .register_type::<RemovedChunkLoaderObservationQueue>()
            .register_type::<RemovedChunkLoaderObservation>();
    }
}
