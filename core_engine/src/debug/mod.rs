pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
use components::DebugObjectComponent;
use resources::{DebugSuiteUIState, DebugSuiteDock};
use systems::{
    perf_ui_startup, toggle_perf_ui_system, debug_object_movement_system, 
    chunk_inspection_system, chunk_loader_inspection_system, chunk_manager_debug_ui, 
    log_registry_debug_ui, render_debug_suite_ui
};
use types::{DebugObjectMovement, StepMode, StepConfig};

use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugSuiteUIState>()
            .init_resource::<DebugSuiteDock>()
            .add_systems(Startup, perf_ui_startup).add_systems(
                Update,
                (
                    toggle_perf_ui_system,
                    debug_object_movement_system.run_if(run_if_not_paused),
                    chunk_inspection_system,
                    chunk_loader_inspection_system,
                    chunk_manager_debug_ui,
                    log_registry_debug_ui,
                ),
            )
            .add_systems(EguiPrimaryContextPass, render_debug_suite_ui)
            .register_type::<DebugObjectComponent>()
            .register_type::<DebugObjectMovement>()
            .register_type::<StepMode>()
            .register_type::<StepConfig>();
    }
}
