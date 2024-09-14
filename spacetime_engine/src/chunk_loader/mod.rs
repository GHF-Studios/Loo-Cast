// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod hooks;
pub mod systems;
pub mod utilities;

// Integrations
pub mod operations;

// Miscelaneous
pub mod constants;

use bevy::prelude::*;

pub(in crate) struct ChunkLoaderPlugin;

impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::RegisteredChunkInfo>()
            .add_systems(Startup, systems::startup);
    }
}