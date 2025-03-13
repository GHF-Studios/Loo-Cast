pub mod components;
pub mod functions;
pub mod systems;

use bevy::prelude::*;
use systems::{
    chunk_inspection_system, chunk_loader_inspection_system, test_object_movement_system,
};

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                test_object_movement_system,
                chunk_inspection_system,
                chunk_loader_inspection_system,
            ),
        );
    }
}
