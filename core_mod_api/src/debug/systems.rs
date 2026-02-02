use crate::bevy::{prelude::*, camera::visibility::RenderLayers};
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts,
};
// TODO: Disabled cause iyes_perf_ui is stuck on bevy 0.16.0
// use iyes_perf_ui::{
//     entries::{
//         diagnostics::{PerfUiEntryFPS, PerfUiEntryFPSAverage},
//         PerfUiSystemEntries,
//     },
//     prelude::{PerfUiEntryEntityCount, PerfUiRoot},
// };

use super::components::DebugObjectComponent;
use super::types::DebugObjectMovement;
use crate::{
    input::states::InputMode,
    logging::resources::LogRegistry,
    render::{
        components::UiCamera,
        // custom_perf_ui_entries::{cursor_position::PerfUiCursorPosEntries, player_position::PerfUiPlayerPosEntries},
        resources::PrimaryWindowUiState,
    },
};

// TODO: Disabled cause iyes_perf_ui is stuck on bevy 0.16.0
// #[tracing::instrument(skip_all)]
// pub(super) fn perf_ui_startup(mut has_spawned: Local<bool>, mut commands: Commands, ui_camera_query: Query<Entity, With<UiCamera>>) {
//     let ui_camera_entity = match ui_camera_query.single() {
//         Ok(entity) => entity,
//         Err(err) => {
//             panic!("Failed to get UiCamera entity for Perf UI setup: {}", err);
//         }
//     };
// 
//     if !*has_spawned {
//         *has_spawned = true;
//         commands.spawn((
//             UiTargetCamera(ui_camera_entity),
//             PerfUiRoot {
//                 fontsize_label: 16.0,
//                 fontsize_value: 16.0,
//                 values_col_width: 256.0,
//                 ..Default::default()
//             },
//             PerfUiEntryFPS::default(),
//             PerfUiEntryFPSAverage::default(),
//             PerfUiSystemEntries::default(),
//             PerfUiEntryEntityCount::default(),
//             // PerfUiPlayerPosEntries::default(),
//             // PerfUiCursorPosEntries::default(),
//             RenderLayers::layer(1),
//         ));
//     }
// }

#[tracing::instrument(skip_all)]
pub(super) fn debug_object_movement_system(time: Res<Time<Virtual>>, mut query: Query<(&mut Transform, &DebugObjectComponent)>) {
    for (mut transform, debug_object) in query.iter_mut() {
        match &debug_object.movement {
            DebugObjectMovement::Static => {}
            DebugObjectMovement::Circle { radius, speed } => {
                let time_factor = time.elapsed_secs() * speed;
                transform.translation.x = radius * time_factor.cos();
                transform.translation.y = radius * time_factor.sin();
            }
            DebugObjectMovement::Line { distance, speed } => {
                let time_factor = time.elapsed_secs() * speed;
                let offset = time_factor.sin() * distance;
                transform.translation.x = offset;
            }
        }
    }
}

// TODO: Move into debug/ui/systems.rs or remove if not needed anymore
#[deprecated]
#[tracing::instrument(skip_all)]
pub(super) fn log_registry_debug_ui(log_registry: Res<LogRegistry>, mut egui_ctx: EguiContexts) {
    // TODO: Remove Temporary stopgap
    return;

    let ctx = match egui_ctx.ctx_mut() {
        Ok(ctx) => ctx,
        Err(_) => {
            return;
        }
    };

    egui::Window::new("Log Registry").vscroll(true).show(ctx, |ui| {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            ui.label(format!("Total Logs: {}", log_registry.logs.len()));
            ui.label(format!("Total Span Selection Roots: {}", log_registry.span_registry.span_roots.len()));
            ui.label(format!("Total Module Selection Roots: {}", log_registry.module_registry.crates.len()));
            ui.label(format!("Total Physical Selection Roots: {}", log_registry.physical_registry.crates.len()));
            //for (key, value) in &log_registry.logs {
            //    ui.label(format!("{}: {:?}", key, value));
            //}
        });
    });
}

#[tracing::instrument(skip_all)]
pub(super) fn toggle_debug_suite_ui_system(
    mut ui_state: ResMut<PrimaryWindowUiState>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    mut next_input_mode: ResMut<NextState<InputMode>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        ui_state.enabled = !ui_state.enabled;
        if ui_state.enabled {
            if input_mode.is_game() {
                next_input_mode.set(InputMode::Debug);
            }

            info!("Debug suite enabled.");
        } else {
            if input_mode.is_debug_suite() {
                next_input_mode.set(InputMode::Release);
            }

            info!("Debug suite disabled.");
        }
    }
}
