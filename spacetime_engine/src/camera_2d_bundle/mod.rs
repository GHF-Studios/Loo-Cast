// Data types
pub mod structs;

// Functions

// Integrations
pub mod operations;

// Miscelaneous

use bevy::prelude::*;

pub(in crate) struct Camera2dBundlePlugin;

impl Plugin for Camera2dBundlePlugin {
    fn build(&self, _app: &mut App) {
    }
}