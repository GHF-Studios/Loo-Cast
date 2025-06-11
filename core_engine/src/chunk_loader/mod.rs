pub mod components;
pub mod observers;
pub mod systems;

pub mod workflows;

use bevy::prelude::*;
use observers::observe_on_remove_chunk_loader;
use systems::update_chunk_loader_system;

use crate::{
    chunk_loader::components::ChunkLoader,
    utils::{cleanup_drop_hooks_system, cleanup_init_hooks_system, observe_on_remove_drop_hook, observe_on_remove_init_hook},
};

pub(crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.observe(observe_on_remove_chunk_loader)
            .observe(observe_on_remove_init_hook::<ChunkLoader>)
            .observe(observe_on_remove_drop_hook::<ChunkLoader>)
            .add_systems(PreUpdate, cleanup_drop_hooks_system::<ChunkLoader>)
            .add_systems(Update, update_chunk_loader_system)
            .add_systems(PostUpdate, cleanup_init_hooks_system::<ChunkLoader>);
    }
}
