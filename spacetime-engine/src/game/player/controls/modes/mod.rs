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

    let (entity, dead, mut noclip, scale_navigation, mut input, mut ground, mut velocity) =
        player.into_inner();
    if dead.is_some() || scale_navigation.is_some() {
        return;
    }

    noclip.active = !noclip.active;
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

/// Selects the appropriate player locomotion model for the active USF scale.
///
/// The metre-authored character motor and local noclip are meaningful only in
/// the local physical domain. Coarser charts use [`PlayerScaleNavigation`]
/// instead. This changes locomotion policy without overloading noclip state.
pub(in crate::game::player) fn sync_scale_navigation_mode(
    mut commands: Commands,
    active: Res<UsfActiveScaleLayer>,
    player: Single<
        (
            Entity,
            &PlayerNoclip,
            Option<&PlayerScaleNavigation>,
        ),
        With<Player>,
    >,
) {
    let (entity, noclip, scale_navigation) = player.into_inner();
    let outside_character_domain = active.scale().exponent() > 4;

    if outside_character_domain {
        if scale_navigation.is_none() {
            commands
                .entity(entity)
                .insert(PlayerScaleNavigation::default());
        }
        commands.entity(entity).remove::<CharacterMotor>();
        return;
    }

    if scale_navigation.is_some() {
        commands.entity(entity).remove::<PlayerScaleNavigation>();
    }

    if noclip.active {
        commands.entity(entity).remove::<CharacterMotor>();
    } else {
        commands.entity(entity).insert(CharacterMotor);
    }
}
