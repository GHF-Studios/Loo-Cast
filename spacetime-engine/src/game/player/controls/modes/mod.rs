//! Human input -> controlled-subject locomotion requests.

use super::*;

fn reset_control_state(input: &mut CharacterMovementInput, ground: &mut CharacterGroundState) {
    input.clear();
    ground.clear_contact();
}

fn spacecraft_flight_active(locomotion: &ControlledSubjectLocomotion) -> bool {
    locomotion.regime() == LocomotionRegime::SpacecraftFlight
}

/// `V` selects whether the ship holds attitude or follows the pilot's view.
pub(in crate::game::player) fn toggle_attitude_law(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (&ControlledSubjectLocomotion, &mut PilotAttitudeLaw),
        With<LocalControlSubject>,
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleAttitudeLaw) {
        return;
    }

    let (locomotion, mut law) = subject.into_inner();
    if dead.into_inner().is_some() || !spacecraft_flight_active(&locomotion) {
        return;
    }
    *law = match *law {
        PilotAttitudeLaw::Hold => PilotAttitudeLaw::FollowView,
        PilotAttitudeLaw::FollowView => PilotAttitudeLaw::Hold,
    };
}

/// `X` toggles the main translational thrusters in spacecraft flight.
///
/// Actuator state belongs to the controlled subject, not to the current
/// detailed/coarse Scale Slice representation.
pub(in crate::game::player) fn toggle_thrusters(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &ControlledSubjectLocomotion,
            &LocomotionCapabilities,
            &mut FlightActuation,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleThrusters) {
        return;
    }

    let (locomotion, capabilities, mut actuation, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some()
        || !spacecraft_flight_active(&locomotion)
        || !capabilities.main_propulsion()
    {
        return;
    }

    let enabled = !actuation.thrusters_enabled();
    actuation.set_thrusters_enabled(enabled);
    reset_control_state(&mut input, &mut ground);
}

/// `Z` toggles bounded RCS stabilization without changing gravity.
pub(in crate::game::player) fn toggle_rcs(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &ControlledSubjectLocomotion,
            &LocomotionCapabilities,
            &mut FlightActuation,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleRcs) {
        return;
    }

    let (locomotion, capabilities, mut actuation, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some()
        || !spacecraft_flight_active(&locomotion)
        || !capabilities.reaction_control()
    {
        return;
    }

    let enabled = !actuation.rcs_enabled();
    actuation.set_rcs_enabled(enabled);
    reset_control_state(&mut input, &mut ground);
}

/// `C` toggles an explicit Cruise request.
pub(in crate::game::player) fn toggle_adaptive_cruise(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &TravelState,
            &LocomotionCapabilities,
            &mut TravelAssistanceState,
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

    let (travel, capabilities, mut assistance, mut cruise, mut input, mut ground) =
        subject.into_inner();
    if dead.into_inner().is_some() {
        return;
    }

    let disabling = assistance.mode() == TravelAssistance::Cruise;

    if disabling {
        assistance.disengage(TravelAssistanceTransitionReason::PilotDisengaged);
    } else {
        if !travel.cruise_entry_available || !capabilities.cruise() {
            return;
        }
        assistance.engage_cruise();
    }

    if disabling {
        cruise.throttle = 0.0;
        cruise.speed_scale0 = 0.0;
    }
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
