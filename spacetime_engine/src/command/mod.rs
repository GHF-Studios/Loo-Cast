// Data types
pub mod structs;
pub mod wrappers;

// Functions

// Integrations

// Miscelaneous
pub mod constants;
pub mod traits;

use bevy::prelude::*;

pub(in crate) struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
    }
}