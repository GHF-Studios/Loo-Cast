//! Adaptation of semantic player death into local control/runtime state.

use crate::game::locomotion::ControlledSubjectLocomotion;

use super::*;

/// Adapts generic semantic death into local-player control state.
///
/// Respawning is intentionally a separate lifecycle mechanic; death cannot be
/// undone by another locomotion request accidentally restoring motion.
pub(super) fn handle_player_death(
    mut commands: Commands,
    mut deaths: MessageReader<Died>,
    player: Single<
        (
            Entity,
            &UsfManifestationOf,
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (
        entity,
        manifestation,
        mut locomotion,
        mut input,
        mut ground,
        mut velocity,
    ) = player.into_inner();

    if !deaths.read().any(|death| death.entity == manifestation.0) {
        return;
    }

    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);
    input.clear();
    ground.grounded = false;
    ground.ground_entity = None;
    velocity.0 = Vec3::ZERO;

    commands
        .entity(entity)
        .remove::<CharacterMotor>()
        .insert(PlayerDead);
}
