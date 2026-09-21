//! Local gameplay-mode and spatial-demand toggles.

use super::*;

/// `V` is the temporary direct binding for the developer `noclip` command.
pub(in crate::game::player) fn toggle_noclip(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            Entity,
            Option<&PlayerDead>,
            &mut PlayerNoclip,
            &mut PlayerAdaptiveCruise,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyV) {
        return;
    }

    let (
        entity,
        dead,
        mut noclip,
        mut cruise,
        mut input,
        mut ground,
        mut velocity,
    ) = player.into_inner();
    if dead.is_some() {
        return;
    }

    noclip.active = !noclip.active;
    if noclip.active {
        cruise.active = false;
        cruise.speed_scale0 = 0.0;
    }
    input.clear();
    velocity.0 = Vec3::ZERO;
    ground.grounded = false;
    ground.ground_entity = None;

    if noclip.active {
        commands.entity(entity).remove::<CharacterMotor>();
        commands.entity(entity).remove::<Collider>();
    }
}

/// `C` toggles adaptive long-distance Cruise.
pub(in crate::game::player) fn toggle_adaptive_cruise(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            Entity,
            Option<&PlayerDead>,
            &mut PlayerAdaptiveCruise,
            &mut PlayerNoclip,
            &PlayerTravelState,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    let (entity, dead, mut cruise, mut noclip, travel, mut input, mut ground, mut velocity) =
        player.into_inner();
    if dead.is_some() {
        return;
    }

    let requested = !cruise.active;
    if requested && travel.critical_dropout {
        return;
    }
    cruise.active = requested;
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;
    noclip.active = false;
    input.clear();
    velocity.0 = Vec3::ZERO;
    ground.grounded = false;
    ground.ground_entity = None;

    if cruise.active {
        commands.entity(entity).remove::<CharacterMotor>();
        commands.entity(entity).remove::<Collider>();
    }
}

/// `L` toggles the player's contribution to generic spatial demand. Other
/// sources (for example Chunkloading Cubes) remain completely independent.
pub(in crate::game::player) fn toggle_spatial_demand(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut player: Single<&mut SpatialDemandSource, With<Player>>,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyL) {
        return;
    }

    player.toggle();
}

/// Keeps exactly one player locomotion implementation authoritative.
///
/// S0 owns the human-scale character controller and collider. Any other active
/// chart uses collisionless scale navigation. Noclip and Cruise also suppress
/// the local character body. Re-entering S0 reconstructs the correct stance
/// collider from canonical human dimensions.
pub(in crate::game::player) fn sync_locomotion_mode(
    mut commands: Commands,
    player: Single<
        (
            Entity,
            &UsfScaleLayer,
            &PlayerStance,
            &PlayerNoclip,
            &PlayerAdaptiveCruise,
            Option<&CharacterMotor>,
            Option<&Collider>,
        ),
        With<Player>,
    >,
) {
    let (entity, layer, stance, noclip, cruise, motor, collider) = player.into_inner();
    let wants_character_body =
        layer.scale() == SpatialScale::ZERO && !noclip.active && !cruise.active;

    if wants_character_body {
        if motor.is_none() {
            commands.entity(entity).insert(CharacterMotor);
        }
        if collider.is_none() {
            let collider = if stance.crouched {
                CharacterDimensions::crouching_collider()
            } else {
                CharacterDimensions::standing_collider()
            };
            commands.entity(entity).insert(collider);
        }
    } else {
        if motor.is_some() {
            commands.entity(entity).remove::<CharacterMotor>();
        }
        if collider.is_some() {
            commands.entity(entity).remove::<Collider>();
        }
    }
}
