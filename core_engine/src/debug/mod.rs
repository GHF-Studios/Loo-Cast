pub mod components;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;
use systems::*;

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, perf_ui_startup).add_systems(
            Update,
            (
                toggle_perf_ui_system,
                debug_object_movement_system,
                chunk_inspection_system,
                chunk_loader_inspection_system,
                chunk_manager_debug_ui,
                log_registry_debug_ui,
            ),
        );
    }
}
