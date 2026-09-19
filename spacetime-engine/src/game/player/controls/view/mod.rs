//! Look and observer-scale input adapters.

use super::*;

pub fn look(
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
pub fn zoom_spatial_view(
    scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut view: ResMut<UsfViewFrame>,
) {
    if gameplay_suppressed(&keyboard, &capture) || scroll.delta.y == 0.0 {
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
    info!(
        scale = %view.scale(),
        fractional_zoom = view.zoom(),
        continuous_scale = view.continuous_exponent(),
        "USF observer scale changed"
    );
}
