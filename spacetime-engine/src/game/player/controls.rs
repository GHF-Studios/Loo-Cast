use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

use crate::physics::character::CharacterMovementInput;

use super::{
    Player,
    PlayerAim,
    PlayerController,
    cursor::CursorCapture,
};

fn gameplay_suppressed(
    keyboard: &ButtonInput<KeyCode>,
    capture: &CursorCapture,
) -> bool {
    !capture.active()
        || keyboard.just_pressed(KeyCode::Tab)
        || keyboard.just_pressed(KeyCode::Escape)
}

pub fn look(
    mouse: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (&PlayerController, &mut PlayerAim),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let (controller, mut aim) = player.into_inner();

    aim.yaw -= mouse.delta.x * controller.look_sensitivity;
    aim.pitch -= mouse.delta.y * controller.look_sensitivity;
    aim.pitch = aim.pitch.clamp(aim.min_pitch, aim.max_pitch);
}

/// Samples local controls once per render frame immediately before the fixed
/// loop. The fixed character motor then consumes this intent deterministically.
pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (&Transform, &PlayerAim, &mut CharacterMovementInput),
        With<Player>,
    >,
) {
    let (body, aim, mut input) = player.into_inner();

    if gameplay_suppressed(&keyboard, &capture) {
        input.clear();
        return;
    }

    let horizontal =
        keyboard.pressed(KeyCode::KeyD) as i8
            - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward =
        keyboard.pressed(KeyCode::KeyW) as i8
            - keyboard.pressed(KeyCode::KeyS) as i8;

    let axis = Vec2::new(horizontal as f32, forward as f32)
        .clamp_length_max(1.0);
    let local_wish = Vec3::new(axis.x, 0.0, -axis.y);
    let world_wish = body.rotation * (aim.yaw_rotation() * local_wish);

    input.set_wish(world_wish, axis.length());
    input.jump_held = keyboard.pressed(KeyCode::Space);
    input.jump_pressed |= keyboard.just_pressed(KeyCode::Space);
}
