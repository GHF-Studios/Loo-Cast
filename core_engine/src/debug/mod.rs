pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use components::DebugObjectComponent;
use resources::{DebugSuiteUiState, DebugSuiteUiDockState};
use systems::{
    perf_ui_startup, toggle_perf_ui_system, debug_object_movement_system, 
    chunk_inspection_system, chunk_loader_inspection_system, chunk_manager_debug_ui, 
    log_registry_debug_ui, debug_suite_ui_system, toggle_debug_suite_ui_system
};
use types::{DebugObjectMovement, StepMode, StepConfig};

use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultInspectorConfigPlugin)
            .init_resource::<DebugSuiteUiState>()
            .init_resource::<DebugSuiteUiDockState>()
            .add_systems(Startup, perf_ui_startup).add_systems(
                Update,
                (
                    toggle_perf_ui_system,
                    toggle_debug_suite_ui_system,
                    debug_object_movement_system.run_if(run_if_not_paused),
                    chunk_inspection_system,
                    chunk_loader_inspection_system,
                    chunk_manager_debug_ui,
                    log_registry_debug_ui,
                ),
            )
            .add_systems(EguiPrimaryContextPass, debug_suite_ui_system)
            .register_type::<DebugObjectComponent>()
            .register_type::<DebugObjectMovement>()
            .register_type::<StepMode>()
            .register_type::<StepConfig>();
    }
}
