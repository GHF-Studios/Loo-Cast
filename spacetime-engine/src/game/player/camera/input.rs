//! Local camera-mode and third-person zoom intent.

use super::*;

pub(in crate::game::player) fn toggle_camera_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut PlayerCamera>,
) {
    if !keyboard.just_pressed(KeyCode::F5) {
        return;
    }

    camera.mode = match camera.mode {
        CameraMode::FirstPerson => CameraMode::ThirdPerson,
        CameraMode::ThirdPerson => CameraMode::FirstPerson,
    };
}

/// Scroll changes persistent zoom intent, never the collision-constrained
/// distance. Wheel-up moves the desired third-person camera inward.
pub(in crate::game::player) fn zoom_third_person(
    scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    presentation: Res<crate::view::PrimaryViewPresentation>,
    mut camera: Single<&mut PlayerCamera>,
) {
    let spatial_zoom = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
    if presentation.is_embedded()
        || spatial_zoom
        || !capture.active()
        || camera.mode != CameraMode::ThirdPerson
        || scroll.delta.y == 0.0
    {
        return;
    }

    camera.third_person.add_zoom_steps(-scroll.delta.y.signum());
}
