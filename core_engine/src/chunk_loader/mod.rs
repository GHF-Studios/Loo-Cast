pub mod components;
pub mod observers;
pub mod resources;
pub mod systems;

pub mod workflows;

use bevy::prelude::*;
use observers::observe_on_remove_chunk_loader;
use systems::update_chunk_loader_system;

use crate::{
    chunk_loader::{components::ChunkLoader, resources::RemovedChunkLoaders},
    utils::{cleanup_drop_hooks_system, cleanup_init_hooks_system, observe_on_remove_drop_hook, observe_on_remove_init_hook},
};

pub(crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RemovedChunkLoaders::default())
            .add_observer(observe_on_remove_chunk_loader)
            .add_observer(observe_on_remove_init_hook::<ChunkLoader>)
            .add_observer(observe_on_remove_drop_hook::<ChunkLoader>)
            .add_systems(PreUpdate, cleanup_drop_hooks_system::<ChunkLoader>)
            .add_systems(Update, update_chunk_loader_system)
            .add_systems(PostUpdate, cleanup_init_hooks_system::<ChunkLoader>);
    }
}
