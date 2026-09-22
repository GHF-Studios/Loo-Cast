//! Look and observer-scale input adapters.

use super::*;

pub(in crate::game::player) fn look(
    mouse: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<(&PlayerController, &mut PlayerAim), With<Player>>,
) {
    if gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let (controller, mut aim) = player.into_inner();
    aim.yaw -= mouse.delta.x * controller.look_sensitivity;
    aim.pitch -= mouse.delta.y * controller.look_sensitivity;
    aim.pitch = aim.pitch.clamp(aim.min_pitch, aim.max_pitch);
}

/// Alt + mouse wheel changes the observer's semantic presentation scale.
pub(in crate::game::player) fn zoom_spatial_view(
    scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    presentation: Res<PrimaryViewPresentation>,
    locomotion: Single<&ControlledSubjectLocomotion, With<Player>>,
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
) {
    if locomotion.regime() == PlayerLocomotionRegime::Cruise
        || presentation.is_embedded()
        || gameplay_suppressed(&keyboard, &capture)
        || scroll.delta.y == 0.0
    {
        return;
    }
    let alt = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
    if !alt {
        return;
    }

    let fast = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let step = if fast { 1.0 } else { 0.1 };
    view.add_zoom(
        -scroll.delta.y.signum() * step,
        SpatialScale::ZERO,
        SpatialScale::MAX,
    );
}
