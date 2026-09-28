//! Look and observer-scale input adapters.

use super::*;

pub(in crate::game::player) fn look(
    input: Res<PlayerInputFrame>,
    profile: Single<&ViewCameraProfile, With<LocalViewTarget>>,
    player: Single<(&PlayerController, &mut PlayerAim), With<Player>>,
) {
    if !input.gameplay_active() || !profile.uses_controller_look() {
        return;
    }

    let (controller, mut aim) = player.into_inner();
    let look = input.look_delta();
    let sensitivity = controller.look_sensitivity.max(0.0);

    // Raw pointer motion is already a per-render-frame accumulated delta. Apply
    // it once without time scaling or smoothing latency.
    aim.yaw = (aim.yaw - look.x * sensitivity + std::f32::consts::PI)
        .rem_euclid(std::f32::consts::TAU)
        - std::f32::consts::PI;
    aim.pitch -= look.y * sensitivity;
    aim.pitch = aim.pitch.clamp(aim.min_pitch, aim.max_pitch);
}

/// Alt + mouse wheel biases automatic semantic presentation.
///
/// The semantic planner remains authoritative, so manual inspection and
/// automatic navigation compose instead of racing over `UsfViewContext`.
pub(in crate::game::player) fn zoom_spatial_view(
    input: Res<PlayerInputFrame>,
    presentation: Res<PrimaryViewPresentation>,
    locomotion: Single<&ControlledSubjectLocomotion, With<LocalControlSubject>>,
    mut state: Single<&mut NavigationPresentationState, With<UsfViewRenderAnchor>>,
) {
    if locomotion.regime() == LocomotionRegime::Cruise
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
