// Data types
pub mod components;
pub mod enums;
pub mod errors;
pub mod structs;
pub mod wrappers;

// Functions
pub mod commands;
pub mod hooks;
pub mod systems;

// Integrations

// Miscelaneous
pub mod constants;
pub mod traits;

use bevy::prelude::*;

pub(in crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::startup);
    }
}