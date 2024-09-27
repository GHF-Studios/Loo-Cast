// Data types

// Functions
pub mod commands;

// Integrations

// Miscelaneous

use bevy::prelude::*;

pub(in crate) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {
    }
}