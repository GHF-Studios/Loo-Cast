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
            Option<&PlayerScaleNavigationNoclip>,
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

    let (entity, dead, mut noclip, scale_forced, mut input, mut ground, mut velocity) =
        player.into_inner();
    if dead.is_some() || scale_forced.is_some() {
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

/// Enforces the validity domain of the metre-authored character motor.
///
/// Above Scale +4 the local character controller is not a meaningful physical
/// model, so the player enters scale-navigation noclip. The marker records
/// whether noclip was forced here or was already a user choice.
pub(in crate::game::player) fn sync_scale_navigation_mode(
    mut commands: Commands,
    active: Res<UsfActiveScaleLayer>,
    player: Single<
        (
            Entity,
            &mut PlayerNoclip,
            Option<&PlayerScaleNavigationNoclip>,
        ),
        With<Player>,
    >,
) {
    let (entity, mut noclip, scale_forced) = player.into_inner();
    let outside_character_domain = active.scale().exponent() > 4;

    if outside_character_domain {
        if !noclip.active {
            noclip.active = true;
            commands
                .entity(entity)
                .insert(PlayerScaleNavigationNoclip);
        }
        commands.entity(entity).remove::<CharacterMotor>();
    } else if scale_forced.is_some() {
        noclip.active = false;
        commands
            .entity(entity)
            .remove::<PlayerScaleNavigationNoclip>()
            .insert(CharacterMotor);
    }
}
