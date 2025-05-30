pub mod components;
pub mod observers;
pub mod systems;

pub mod workflows;

use bevy::prelude::*;
use observers::observe_on_remove_chunk_loader;
use systems::update_chunk_loader_system;

pub(crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_chunk_loader_system).observe(observe_on_remove_chunk_loader);
    }
}
