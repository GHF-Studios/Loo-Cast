//! Human input -> controlled-subject locomotion requests.

use super::*;

fn spacecraft_flight_active(locomotion: &ControlledSubjectLocomotion) -> bool {
    locomotion.regime() == LocomotionRegime::SpacecraftFlight
}

/// `V` selects whether the ship holds attitude or follows the pilot's view.
pub(in crate::game::player) fn toggle_attitude_law(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (Entity, &ControlledSubjectLocomotion, &PilotAttitudeLaw),
        With<LocalControlSubject>,
    >,
    mut requests: MessageWriter<FlightControlRequest>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleAttitudeLaw) {
        return;
    }

    let (entity, locomotion, law) = subject.into_inner();
    if dead.into_inner().is_some() || !spacecraft_flight_active(locomotion) {
        return;
    }
    let requested = match *law {
        PilotAttitudeLaw::Hold => PilotAttitudeLaw::FollowView,
        PilotAttitudeLaw::FollowView => PilotAttitudeLaw::Hold,
    };
    requests.write(FlightControlRequest::new(
        entity,
        FlightControlCommand::SetPilotAttitudeLaw(requested),
    ));
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
            Entity,
            &ControlledSubjectLocomotion,
            &FlightCapabilities,
            &FlightActuation,
        ),
        With<LocalControlSubject>,
    >,
    mut requests: MessageWriter<FlightControlRequest>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleThrusters) {
        return;
    }

    let (entity, locomotion, capabilities, actuation) = subject.into_inner();
    if dead.into_inner().is_some()
        || !spacecraft_flight_active(locomotion)
        || !capabilities.main_propulsion()
    {
        return;
    }

    requests.write(FlightControlRequest::new(
        entity,
        FlightControlCommand::SetMainPropulsion(!actuation.thrusters_enabled()),
    ));
}

/// `Z` toggles bounded RCS stabilization without changing gravity.
pub(in crate::game::player) fn toggle_rcs(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            Entity,
            &ControlledSubjectLocomotion,
            &FlightCapabilities,
            &FlightActuation,
        ),
        With<LocalControlSubject>,
    >,
    mut requests: MessageWriter<FlightControlRequest>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleRcs) {
        return;
    }

    let (entity, locomotion, capabilities, actuation) = subject.into_inner();
    if dead.into_inner().is_some()
        || !spacecraft_flight_active(locomotion)
        || !capabilities.reaction_control()
    {
        return;
    }

    requests.write(FlightControlRequest::new(
        entity,
        FlightControlCommand::SetReactionControl(!actuation.rcs_enabled()),
    ));
}

/// `C` toggles an explicit Cruise request.
pub(in crate::game::player) fn toggle_adaptive_cruise(
    input: Res<PlayerInputFrame>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<Entity, With<LocalControlSubject>>,
    mut requests: MessageWriter<TravelAssistanceRequest>,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::ToggleCruise) {
        return;
    }
    if dead.into_inner().is_some() {
        return;
    }
    requests.write(TravelAssistanceRequest::toggle_cruise(subject.into_inner()));
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
