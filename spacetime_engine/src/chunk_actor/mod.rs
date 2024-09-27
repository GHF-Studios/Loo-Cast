// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod hooks;
pub mod systems;
pub mod utilities;

// Integrations
pub mod commands;
pub mod operations;

// Miscelaneous
pub mod constants;

use bevy::prelude::*;

pub(in crate) struct ChunkActorPlugin;

impl Plugin for ChunkActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<components::ChunkActor>()
            .add_systems(Startup, systems::startup);
    }
}