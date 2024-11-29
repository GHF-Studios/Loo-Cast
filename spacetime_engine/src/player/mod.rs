use bevy::prelude::*;

// Data types
pub mod components;
pub mod wrappers;

// Functions
pub mod commands;

// Miscelaneous
pub mod constants;

pub(in crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
    }
}