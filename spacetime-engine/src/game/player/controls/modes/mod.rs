//! Human input -> controlled-subject locomotion requests.

use super::*;

fn reset_control_state(
    input: &mut CharacterMovementInput,
    ground: &mut CharacterGroundState,
) {
    input.clear();
    ground.clear_contact();
}

/// `V` toggles an explicit Local Flight request.
pub(in crate::game::player) fn toggle_local_flight(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleLocalFlight) {
        return;
    }

    let (mut locomotion, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some() {
        return;
    }

    if locomotion.request() == LocomotionRequest::Regime(LocomotionRegime::LocalFlight) {
        locomotion.request_automatic();
        locomotion.set_thrusters_enabled(false);
    } else {
        locomotion.request_regime(LocomotionRegime::LocalFlight);
        locomotion.set_thrusters_enabled(true);
    }

    reset_control_state(&mut input, &mut ground);
}

/// `X` toggles translational thrusters inside detailed-slice Local Flight.
pub(in crate::game::player) fn toggle_local_flight_thrusters(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &UsfScaleLayer,
            &DetailedBodyScale,
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleThrusters) {
        return;
    }

    let (layer, detailed, mut locomotion, mut input, mut ground) = subject.into_inner();

    if dead.into_inner().is_some()
        || layer.scale() != detailed.0
        || (locomotion.regime() != LocomotionRegime::LocalFlight
            && locomotion.request()
                != LocomotionRequest::Regime(LocomotionRegime::LocalFlight))
    {
        return;
    }

    let enabled = !locomotion.thrusters_enabled();
    locomotion.set_thrusters_enabled(enabled);
    reset_control_state(&mut input, &mut ground);
}

/// `C` toggles an explicit Cruise request.
pub(in crate::game::player) fn toggle_adaptive_cruise(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &TravelState,
            &mut ControlledSubjectLocomotion,
            &mut AdaptiveCruise,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleCruise) {
        return;
    }

    let (travel, mut locomotion, mut cruise, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some() {
        return;
    }

    let disabling =
        locomotion.request() == LocomotionRequest::Regime(LocomotionRegime::Cruise);

    if disabling {
        locomotion.request_automatic();
    } else {
        if travel.critical_dropout {
            return;
        }
        locomotion.request_regime(LocomotionRegime::Cruise);
    }

    locomotion.set_thrusters_enabled(false);
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;
    reset_control_state(&mut input, &mut ground);
}

/// `L` toggles the controlled subject's contribution to generic spatial demand.
pub(in crate::game::player) fn toggle_spatial_demand(
    input: Res<PlayerInputFrame>,
    mut subject: Single<&mut SpatialDemandSource, With<LocalControlSubject>>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleSpatialDemand) {
        return;
    }

    subject.toggle();
}
