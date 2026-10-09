//! Look and observer-scale input adapters.

use super::*;

pub(in crate::game::player) fn write_player_view_intent(
    input: Res<PlayerInputFrame>,
    mut camera: Single<&mut PlayerCamera>,
    subject: Single<&MotionExecution, With<LocalControlSubject>>,
    player: Single<(&PlayerController, &mut PlayerAim), With<Player>>,
) {
    if !input.gameplay_active() {
        return;
    }

    let (controller, mut aim) = player.into_inner();
    let look = input.look_delta();
    let sensitivity = controller.look_sensitivity.max(0.0);

    if camera.mode == CameraMode::Orbit {
        camera.orbit_rotation = (Quat::from_rotation_y(-look.x * sensitivity)
            * camera.orbit_rotation
            * Quat::from_rotation_x(-look.y * sensitivity))
        .normalize();
        return;
    }
    if subject.kernel() == MotionKernel::InertialFlight {
        return;
    }

    // Raw pointer motion is already a per-render-frame accumulated delta. Apply
    // it once without time scaling or smoothing latency.
    aim.yaw = (aim.yaw - look.x * sensitivity + std::f32::consts::PI)
        .rem_euclid(std::f32::consts::TAU)
        - std::f32::consts::PI;
    aim.pitch -= look.y * sensitivity;
    aim.pitch = aim.pitch.clamp(aim.min_pitch, aim.max_pitch);
    aim.roll = 0.0;
}

/// Alt + mouse wheel biases automatic semantic presentation.
///
/// The semantic planner remains authoritative, so manual inspection and
/// automatic navigation compose instead of racing over `UsfViewContext`.
pub(in crate::game::player) fn adjust_view_scale_bias(
    input: Res<PlayerInputFrame>,
    presentation: Res<PrimaryViewPresentation>,
    assistance: Single<&TravelAssistanceState, With<LocalControlSubject>>,
    mut state: Single<&mut NavigationPresentationState, With<UsfViewRenderAnchor>>,
) {
    if assistance.mode() == TravelAssistance::Cruise
        || presentation.is_embedded()
        || !input.gameplay_active()
        || input.scroll_y() == 0.0
    {
        return;
    }
    if !input.pressed(PlayerAction::ViewScaleModifier) {
        return;
    }

    let step = if input.pressed(PlayerAction::FastModifier) {
        1.0
    } else {
        0.1
    };
    state.add_manual_bias(-input.scroll_y().signum() * step);
}
