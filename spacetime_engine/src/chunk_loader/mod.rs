pub mod components;
pub mod events;
pub mod hooks;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use systems::update_chunk_loader_system;
use resources::ChunkOwnership;

pub(in crate) struct ChunkLoaderPlugin;
impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkOwnership::default())
            .add_systems(Update, update_chunk_loader_system);
    }
}