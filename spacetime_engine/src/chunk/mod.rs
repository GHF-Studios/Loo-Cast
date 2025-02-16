pub mod bundles;
pub mod components;
pub mod enums;
pub mod errors;
pub mod functions;
pub mod resources;
pub mod systems;

pub mod actions;

use bevy::prelude::*;
use resources::{ChunkActionBuffer, ChunkManager};
use systems::{process_chunk_actions, startup_chunk_system, update_chunk_system};

pub(in crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkActionBuffer::default())
            .insert_resource(ChunkManager::default())
            .add_systems(Startup, startup_chunk_system)
            .add_systems(Update, update_chunk_system)
            .add_systems(PostUpdate, process_chunk_actions);
    }
}