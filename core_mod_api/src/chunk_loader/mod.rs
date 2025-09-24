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

use crate::core_mod_macros::configure_app_with_all_scales;
use crate::{
    core::run_conditions::run_after_startup_finished,
    time::run_conditions::run_if_not_paused,
    usf::scale::*,
    utils::{
        components::{DropHook, InitHook},
        functions::{cleanup_drop_hooks_system, cleanup_init_hooks_system, observe_on_remove_drop_hook, observe_on_remove_init_hook},
    },
};

pub(crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_chunk_loader_system, 
                on_remove_chunk_loader_observation_queue_processing_system,
            ).run_if(run_after_startup_finished.and(run_if_not_paused)))
            
            .insert_resource(RemovedChunkLoaderObservationQueue::default())

            .register_type::<RemovedChunkLoaderObservationQueue>()
            .register_type::<RemovedChunkLoaderObservation>();
        
        configure_app_with_all_scales!(
            { .insert_resource(RemovedChunkLoaders::<__S__>::default()) },
            { .add_observer(observe_on_remove_chunk_loader::<__S__>) },
            { .add_observer(observe_on_remove_init_hook::<ChunkLoader<__S__>>) },
            { .add_observer(observe_on_remove_drop_hook::<ChunkLoader<__S__>>) },
            { .add_systems(
                PreUpdate,
                cleanup_drop_hooks_system::<ChunkLoader<__S__>>.run_if(run_after_startup_finished.and(run_if_not_paused)),
            ) },
            { .add_systems(
                PostUpdate,
                (cleanup_init_hooks_system::<ChunkLoader<__S__>>, post_update_chunk_loader_system::<__S__>).run_if(run_after_startup_finished.and(run_if_not_paused)),
            ) },
            { .register_type::<ChunkLoader<__S__>>() },
            { .register_type::<RemovedChunkLoader<__S__>>() },
            { .register_type::<RemovedChunkLoaders<__S__>>() },
            { .register_type::<InitHook<ChunkLoader<__S__>>>() },
            { .register_type::<DropHook<ChunkLoader<__S__>>>() },
        );
    }
}
