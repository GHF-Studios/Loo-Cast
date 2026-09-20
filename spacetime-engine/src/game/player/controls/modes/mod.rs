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
            Option<&PlayerScaleNavigation>,
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
        scale_navigation,
        mut input,
        mut ground,
        mut velocity,
    ) = player.into_inner();
    if dead.is_some() || scale_navigation.is_some() {
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
    } else {
        commands.entity(entity).insert(CharacterMotor);
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

    let (entity, dead, mut cruise, mut noclip, mut input, mut ground, mut velocity) =
        player.into_inner();
    if dead.is_some() {
        return;
    }

    cruise.active = !cruise.active;
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;
    noclip.active = false;
    input.clear();
    velocity.0 = Vec3::ZERO;
    ground.grounded = false;
    ground.ground_entity = None;

    if cruise.active {
        commands.entity(entity).remove::<CharacterMotor>();
    } else {
        commands.entity(entity).insert(CharacterMotor);
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
/// Scale does not disable physics: `CharacterMotor` uses ordinary numeric local
/// units in whatever [`UsfScaleLayer`] currently owns the player. Developer
/// noclip and explicit [`PlayerScaleNavigation`] are alternative locomotion
/// modes, not automatic consequences of entering a coarse scale.
pub(in crate::game::player) fn sync_locomotion_mode(
    mut commands: Commands,
    player: Single<
        (
            Entity,
            &PlayerNoclip,
            &PlayerAdaptiveCruise,
            Option<&PlayerScaleNavigation>,
            Option<&CharacterMotor>,
        ),
        With<Player>,
    >,
) {
    let (entity, noclip, cruise, scale_navigation, motor) = player.into_inner();
    let wants_character_motor = !noclip.active && !cruise.active && scale_navigation.is_none();

    match (wants_character_motor, motor.is_some()) {
        (true, false) => {
            commands.entity(entity).insert(CharacterMotor);
        }
        (false, true) => {
            commands.entity(entity).remove::<CharacterMotor>();
        }
        _ => {}
    }
}
