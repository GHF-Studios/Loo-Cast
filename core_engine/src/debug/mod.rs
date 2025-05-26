pub mod components;
pub mod systems;

pub mod workflows;

use bevy::prelude::*;
use systems::{
    chunk_inspection_system, chunk_loader_inspection_system, debug_object_movement_system,
    debug_ui_startup,
};

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_ui_startup).add_systems(
            Update,
            (
                debug_object_movement_system,
                chunk_inspection_system,
                chunk_loader_inspection_system,
            ),
        );
    }
}
