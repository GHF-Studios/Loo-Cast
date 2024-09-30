// Data types
pub mod structs;

// Functions
pub mod commands;

// Integrations

// Miscelaneous
pub mod traits;

use bevy::prelude::*;

pub(in crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {
    }
}