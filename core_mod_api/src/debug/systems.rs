use crate::{
    camera::{components::MainCamera, resources::GameViewRenderTarget},
    debug::resources::{DebugSuiteUiDockState, DebugSuiteUiState},
    input::states::InputMode,
    logging::resources::LogRegistry,
    ui::{custom_perf_ui_entries::{cursor_position::PerfUiCursorPosEntries, player_position::PerfUiPlayerPosEntries}, toolbar::resources::ToolbarState},
};

use bevy::{
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
    window::WindowRef,
};
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts,
};
use iyes_perf_ui::{
    prelude::PerfUiRoot,
    entries::diagnostics::{PerfUiEntryFPS, PerfUiEntryFPSAverage},
};

use super::components::DebugObjectComponent;
use super::types::DebugObjectMovement;

#[tracing::instrument(skip_all)]
pub(super) fn perf_ui_startup(mut has_spawned: Local<bool>, mut commands: Commands) {
    use iyes_perf_ui::{
        entries::PerfUiSystemEntries,
        prelude::{PerfUiEntryEntityCount, PerfUiRoot},
    };

    if !*has_spawned {
        *has_spawned = true;
        commands.spawn((
            PerfUiRoot::default(),
            PerfUiEntryFPS::default(),
            PerfUiEntryFPSAverage::default(),
            PerfUiSystemEntries::default(),
            PerfUiEntryEntityCount::default(),
            PerfUiPlayerPosEntries::default(),
            PerfUiCursorPosEntries::default(),
            RenderLayers::layer(1),
        ));
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn toggle_perf_ui_system(mut query: Query<&mut Visibility, With<PerfUiRoot>>, toolbar_state: Res<ToolbarState>) {
    for mut vis in query.iter_mut() {
        match (*vis, toolbar_state.show_perf_ui) {
            (Visibility::Inherited, false) => {
                *vis = Visibility::Hidden;
            }
            (Visibility::Inherited, true) => {
                *vis = Visibility::Visible;
            }
            (Visibility::Hidden, false) => {}
            (Visibility::Hidden, true) => {
                *vis = Visibility::Visible;
            }
            (Visibility::Visible, false) => {
                *vis = Visibility::Hidden;
            }
            (Visibility::Visible, true) => {}
        }
    }
}

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
#[tracing::instrument(skip_all)]
pub(super) fn log_registry_debug_ui(log_registry: Res<LogRegistry>, mut egui_ctx: EguiContexts, toolbar_state: Res<ToolbarState>) {
    if !toolbar_state.show_log_registry_debug_ui {
        return;
    }
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
    render_target: Res<GameViewRenderTarget>,
    mut ui_state: ResMut<DebugSuiteUiState>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    mut next_input_mode: ResMut<NextState<InputMode>>,
    mut main_camera_query: Single<&mut Camera, With<MainCamera>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        ui_state.enabled = !ui_state.enabled;
        if ui_state.enabled {
            main_camera_query.target = RenderTarget::Image(render_target.handle.clone().into());

            if input_mode.is_game() {
                next_input_mode.set(InputMode::DebugSuite);
            }

            info!("Debug suite UI enabled.");
        } else {
            main_camera_query.target = RenderTarget::Window(WindowRef::Primary);

            if input_mode.is_debug_suite() {
                next_input_mode.set(InputMode::Game);
            }

            info!("Debug suite UI disabled.");
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn debug_suite_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy_egui::PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<DebugSuiteUiState, _>(|world, mut state| {
        world.resource_scope::<DebugSuiteUiDockState, _>(|world, mut dock_state| {
            world.resource_scope::<GameViewRenderTarget, _>(|world, target| {
                super::functions::draw_debug_suite(&mut state, &mut dock_state, &target, world, egui_context.get_mut());
            });
        });
    });
}
