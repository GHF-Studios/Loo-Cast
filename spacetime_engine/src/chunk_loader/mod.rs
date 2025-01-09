pub mod components;
pub mod constants;
pub mod observers;
pub mod systems;

use bevy::prelude::*;
use observers::{observe_on_add_chunk_loader, observe_on_remove_chunk_loader};
use resources::ChunkActionBuffer;
use systems::{process_chunk_retry_queue_system, update_chunk_loader_system};

pub(in crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_chunk_loader_system)
            .observe(observe_on_add_chunk_loader)
            .observe(observe_on_remove_chunk_loader);
    }
}