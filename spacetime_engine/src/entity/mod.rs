// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod commands;
pub mod hooks;
pub mod systems;

// Integrations
pub mod operations;

use bevy::prelude::*;

pub(in crate) struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::EntityPosition>()
            .add_systems(Startup, systems::startup);
    }
}