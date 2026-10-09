//! Local camera-mode and third-person zoom intent.

use super::*;

use crate::game::control::LocalControlSubject;

pub(in crate::game::player) fn toggle_camera_mode(
    input: Res<PlayerInputFrame>,
    profile: Single<&ViewCameraProfile, With<LocalViewTarget>>,
    mut camera: Single<(&mut PlayerCamera, &Transform)>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleCameraMode) {
        return;
    }

    let (camera, transform) = &mut *camera;
    let next = match camera.mode {
        CameraMode::FirstPerson => CameraMode::ThirdPerson,
        CameraMode::ThirdPerson => {
            if profile.supports_mode(CameraMode::Orbit) {
                CameraMode::Orbit
            } else {
                CameraMode::FirstPerson
            }
        }
        CameraMode::Orbit => CameraMode::FirstPerson,
    };
    if next == CameraMode::Orbit {
        camera.orbit_rotation = transform.rotation;
    }
    camera.mode = next;
}

/// Scroll changes persistent zoom intent, never the collision-constrained
/// distance. Wheel-up moves the desired third-person camera inward.
pub(in crate::game::player) fn zoom_third_person(
    input: Res<PlayerInputFrame>,
    presentation: Res<crate::view::PrimaryViewPresentation>,
    camera: Single<&PlayerCamera>,
    controlled_vehicle: Query<(), (With<LocalControlSubject>, Without<Player>)>,
    mut profile: Single<&mut ViewCameraProfile, With<LocalViewTarget>>,
) {
    if presentation.is_embedded()
        || (!controlled_vehicle.is_empty() && !input.pressed(PlayerAction::Descend))
        || input.pressed(PlayerAction::ViewScaleModifier)
        || !input.gameplay_active()
        || !matches!(camera.mode, CameraMode::ThirdPerson | CameraMode::Orbit)
        || input.scroll_y() == 0.0
    {
        return;
    }

    profile
        .third_person
        .add_zoom_steps(-input.scroll_y().signum());
}
