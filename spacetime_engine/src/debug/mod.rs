pub mod components;
pub mod systems;

pub mod workflows;

// pub mod workflows_MACROINPUT;
// pub mod workflows_MACROOUTPUT;

use bevy::prelude::*;
use systems::{
    chunk_inspection_system, chunk_loader_inspection_system, debug_object_movement_system,
};

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                debug_object_movement_system,
                chunk_inspection_system,
                chunk_loader_inspection_system,
            ),
        );
    }
}
