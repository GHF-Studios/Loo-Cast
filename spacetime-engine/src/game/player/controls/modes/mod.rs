//! Human input -> controlled-subject locomotion requests.

use super::*;

fn reset_control_state(
    input: &mut CharacterMovementInput,
    ground: &mut CharacterGroundState,
) {
    input.clear();
    ground.clear_contact();
}

fn local_flight_active_or_requested(locomotion: &ControlledSubjectLocomotion) -> bool {
    locomotion.regime() == LocomotionRegime::LocalFlight
        || locomotion.request() == LocomotionRequest::Regime(LocomotionRegime::LocalFlight)
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
        locomotion.set_rcs_enabled(false);
    } else {
        locomotion.request_regime(LocomotionRegime::LocalFlight);
        locomotion.set_thrusters_enabled(true);
        locomotion.set_rcs_enabled(true);
    }

    reset_control_state(&mut input, &mut ground);
}

/// `X` toggles the main translational thrusters in Local Flight.
///
/// Actuator state belongs to the controlled subject, not to the current
/// detailed/coarse Scale Slice representation.
pub(in crate::game::player) fn toggle_local_flight_thrusters(
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
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleThrusters) {
        return;
    }

    let (mut locomotion, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some() || !local_flight_active_or_requested(&locomotion) {
        return;
    }

    let enabled = !locomotion.thrusters_enabled();
    locomotion.set_thrusters_enabled(enabled);
    reset_control_state(&mut input, &mut ground);
}

/// `Z` toggles local-flight RCS stabilization.
///
/// RCS is an actuator/response policy: while enabled, local inertial flight
/// ignores sampled gravity for this craft and applies bounded thrust damping
/// whenever main translational thrust is not actively commanded.
pub(in crate::game::player) fn toggle_local_flight_rcs(
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
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleRcs) {
        return;
    }

    let (mut locomotion, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some() || !local_flight_active_or_requested(&locomotion) {
        return;
    }

    let enabled = !locomotion.rcs_enabled();
    locomotion.set_rcs_enabled(enabled);
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
    locomotion.set_rcs_enabled(false);
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
