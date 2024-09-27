// Data types

// Functions

// Integrations
pub mod commands;

// Miscelaneous

use bevy::prelude::*;

pub(in crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {
    }
}