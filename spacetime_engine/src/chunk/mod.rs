// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod commands;
pub mod hooks;
pub mod systems;
pub mod utilities;

// Integrations
pub mod operations;

// Miscelaneous
pub mod constants;
pub mod singletons;

use bevy::prelude::*;

pub(in crate) struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::startup)
            .register_type::<structs::ChunkPosition>();
    }
}