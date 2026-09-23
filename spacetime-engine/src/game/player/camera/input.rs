//! Local camera-mode and third-person zoom intent.

use super::*;

pub(in crate::game::player) fn toggle_camera_mode(
    input: Res<PlayerInputFrame>,
    mut camera: Single<&mut PlayerCamera>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleCameraMode) {
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
    input: Res<PlayerInputFrame>,
    presentation: Res<crate::view::PrimaryViewPresentation>,
    camera: Single<&PlayerCamera>,
    mut profile: Single<&mut ViewCameraProfile, With<LocalViewTarget>>,
) {
    if presentation.is_embedded()
        || input.pressed(PlayerAction::ViewScaleModifier)
        || !input.gameplay_active()
        || camera.mode != CameraMode::ThirdPerson
        || input.scroll_y() == 0.0
    {
        return;
    }

    profile
        .third_person
        .add_zoom_steps(-input.scroll_y().signum());
}
