pub mod components;
pub mod functions;
pub mod gizmo;
pub mod selection;
pub mod systems;
pub mod types;

use crate::bevy::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use components::DebugObjectComponent;
use gizmo::GizmoPlugin;
use selection::SelectionPlugin;
use systems::{
    debug_object_movement_system,
    log_registry_debug_ui,
    // perf_ui_startup, // TODO: Disabled cause iyes_perf_ui is stuck on bevy 0.16.0
    toggle_debug_suite_ui_system
};
use types::{DebugObjectMovement, DebugSuiteTab, InspectorSelection, StepConfig, StepMode};

use crate::{core::run_conditions::run_after_startup_finished, time::run_conditions::run_if_not_paused};

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(GizmoPlugin)
            .add_plugins(SelectionPlugin)
            // .add_systems(Startup, perf_ui_startup) // TODO: Disabled cause iyes_perf_ui is stuck on bevy 0.16.0
            .add_systems(
                Update,
                (
                    toggle_debug_suite_ui_system,
                    debug_object_movement_system.run_if(run_after_startup_finished.and(run_if_not_paused)),
                    log_registry_debug_ui,
                ),
            )
            .register_type::<DebugObjectComponent>()
            .register_type::<DebugObjectMovement>()
            .register_type::<StepMode>()
            .register_type::<StepConfig>()
            .register_type::<DebugSuiteTab>()
            .register_type::<InspectorSelection>();
    }
}
