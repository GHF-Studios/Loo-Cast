pub mod components;
pub mod observers;
pub mod resources;
pub mod systems;

pub mod workflows;

use bevy::prelude::*;
use components::ChunkLoader;
use observers::observe_on_remove_chunk_loader;
use resources::{RemovedChunkLoader, RemovedChunkLoaders};
use systems::update_chunk_loader_system;

use crate::{
    time::run_conditions::run_if_not_paused,
    utils::{
        components::{DropHook, InitHook},
        functions::{cleanup_drop_hooks_system, cleanup_init_hooks_system, observe_on_remove_drop_hook, observe_on_remove_init_hook},
    },
};

pub(crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RemovedChunkLoaders::default())
            .add_observer(observe_on_remove_chunk_loader)
            .add_observer(observe_on_remove_init_hook::<ChunkLoader>)
            .add_observer(observe_on_remove_drop_hook::<ChunkLoader>)
            .add_systems(PreUpdate, cleanup_drop_hooks_system::<ChunkLoader>.run_if(run_if_not_paused))
            .add_systems(Update, update_chunk_loader_system.run_if(run_if_not_paused))
            .add_systems(PostUpdate, cleanup_init_hooks_system::<ChunkLoader>.run_if(run_if_not_paused))
            .register_type::<ChunkLoader>()
            .register_type::<RemovedChunkLoader>()
            .register_type::<RemovedChunkLoaders>()
            .register_type::<InitHook<ChunkLoader>>()
            .register_type::<DropHook<ChunkLoader>>();
    }
}
