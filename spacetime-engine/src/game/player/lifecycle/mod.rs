//! Adaptation of semantic player death into local control/runtime state.

use crate::game::{
    control::LocalControlSubject,
    flight::{FlightControlCommand, FlightControlRequest},
    locomotion::{ControlledSubjectLocomotion, FlightControlIntent, LocomotionEnabled},
};

use crate::spatial::UsfCanonicalMotion;

use super::*;

/// Adapts generic semantic death into the currently controlled runtime subject.
///
/// Player identity and controlled subject are intentionally independent: dying
/// while piloting a spacecraft must affect the spacecraft control runtime, not
/// only the hidden player-body manifestation.
pub(super) fn handle_player_death(
    mut commands: Commands,
    mut deaths: MessageReader<Died>,
    mut flight_requests: MessageWriter<FlightControlRequest>,
    ownership: UsfOwnershipQuery,
    player: Single<Entity, With<Player>>,
    subject: Single<
        (
            Entity,
            &mut LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
            &mut FlightControlIntent,
            &mut CharacterMovementIntent,
            &mut CharacterGroundState,
            &mut UsfCanonicalMotion,
            Option<&mut LinearVelocity>,
        ),
        With<LocalControlSubject>,
    >,
) {
    let player_entity = player.into_inner();
    let Some(player_semantic) = ownership.semantic_of(player_entity) else {
        error!(
            realization = ?player_entity,
            "player realization has no semantic USF owner during death handling"
        );
        return;
    };

    if !deaths.read().any(|death| death.entity == player_semantic) {
        return;
    }

    let (
        subject_entity,
        mut enabled,
        mut locomotion,
        mut flight_intent,
        mut input,
        mut ground,
        mut motion,
        velocity,
    ) = subject.into_inner();

    enabled.0 = false;
    locomotion.request_automatic();
    flight_requests.write(FlightControlRequest::new(
        subject_entity,
        FlightControlCommand::SetMainPropulsion(false),
    ));
    flight_requests.write(FlightControlRequest::new(
        subject_entity,
        FlightControlCommand::SetReactionControl(false),
    ));
    flight_intent.clear();
    input.clear();
    ground.clear_contact();
    motion.stop();

    if let Some(mut velocity) = velocity {
        velocity.0 = Vec3::ZERO;
    }

    commands.entity(subject_entity).remove::<CharacterMotor>();
    commands.entity(player_entity).insert(PlayerDead);
}
