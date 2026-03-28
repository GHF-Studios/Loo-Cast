use crate::bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};
use crate::render::components::{EguiCamera, MainCamera, UiCamera};

#[derive(Resource)]
pub(super) struct PendingWindowModeTransition {
    pub target_mode: WindowMode,
    pub phase: WindowModeTransitionPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WindowModeTransitionPhase {
    DeactivateCameras,
    ApplyWindowMode,
    ReactivateCameras,
}

#[tracing::instrument(skip_all)]
pub(super) fn queue_window_mode_toggle(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    transition: Option<Res<PendingWindowModeTransition>>,
) {
    if keys.just_pressed(KeyCode::F11) && transition.is_none() {
        let target_mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            _ => WindowMode::Windowed,
        };
        commands.insert_resource(PendingWindowModeTransition {
            target_mode,
            phase: WindowModeTransitionPhase::DeactivateCameras,
        });
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn process_window_mode_transition(
    mut commands: Commands,
    mut transition: Option<ResMut<PendingWindowModeTransition>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Camera, Or<(With<MainCamera>, With<UiCamera>, With<EguiCamera>)>>,
) {
    let Some(mut transition) = transition else {
        return;
    };

    match transition.phase {
        WindowModeTransitionPhase::DeactivateCameras => {
            for mut camera in camera_query.iter_mut() {
                camera.is_active = false;
            }
            transition.phase = WindowModeTransitionPhase::ApplyWindowMode;
        }
        WindowModeTransitionPhase::ApplyWindowMode => {
            window.mode = transition.target_mode;
            transition.phase = WindowModeTransitionPhase::ReactivateCameras;
        }
        WindowModeTransitionPhase::ReactivateCameras => {
            for mut camera in camera_query.iter_mut() {
                camera.is_active = true;
            }
            commands.remove_resource::<PendingWindowModeTransition>();
        }
    }
}
