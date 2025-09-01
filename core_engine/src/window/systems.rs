use bevy::{prelude::*, window::{PrimaryWindow, WindowMode}};

#[tracing::instrument(skip_all)]
pub(super) fn toggle_window_mode(mut window: Single<&mut Window, With<PrimaryWindow>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F11) {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            _ => WindowMode::Windowed,
        };
    }
}