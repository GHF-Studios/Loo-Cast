//! Local input adapters for the player body.
//!
//! These systems translate devices into components understood by lower-level
//! simulation. They intentionally do not own camera collision, item semantics
//! or movement tuning.

use avian3d::prelude::LinearVelocity;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::physics::character::{
    CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame, CharacterMotor,
    CharacterMovementInput,
};

use super::{
    Player, PlayerAim, PlayerController, PlayerNoclip, PlayerStance, cursor::CursorCapture,
};

pub(super) fn gameplay_suppressed(
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
    player: Single<(&PlayerController, &mut PlayerAim), With<Player>>,
) {
    if gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let (controller, mut aim) = player.into_inner();
    aim.yaw -= mouse.delta.x * controller.look_sensitivity;
    aim.pitch -= mouse.delta.y * controller.look_sensitivity;
    aim.pitch = aim.pitch.clamp(aim.min_pitch, aim.max_pitch);
}

/// `V` is the temporary direct binding for the developer `noclip` command.
pub fn toggle_noclip(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            Entity,
            &mut PlayerNoclip,
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

    let (entity, mut noclip, mut input, mut ground, mut velocity) = player.into_inner();

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

/// Samples local controls once per render frame immediately before the fixed
/// loop. The fixed character motor consumes this intent deterministically.
pub fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            &PlayerAim,
            &PlayerController,
            &PlayerStance,
            &PlayerNoclip,
            &mut CharacterMovementInput,
        ),
        With<Player>,
    >,
) {
    let (frame, control, aim, controller, stance, noclip, mut input) = player.into_inner();

    if gameplay_suppressed(&keyboard, &capture) || noclip.active {
        input.clear();
        return;
    }

    let sprinting = !stance.crouched
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight));

    let speed_multiplier = if stance.crouched {
        controller.crouch_speed_multiplier
    } else if sprinting {
        controller.sprint_multiplier
    } else {
        1.0
    };

    let horizontal = keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward = keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;

    let axis = Vec2::new(horizontal as f32, forward as f32).clamp_length_max(1.0);
    let up = frame.up();
    let view_forward = control.rotation() * (aim.yaw_rotation() * Vec3::NEG_Z);
    let planar_forward = view_forward - up * view_forward.dot(up);
    let forward = if planar_forward.length_squared() > 1.0e-8 {
        planar_forward.normalize()
    } else {
        frame.aligned_rotation(control.rotation()) * Vec3::NEG_Z
    };
    let right = forward.cross(up).normalize_or_zero();
    let world_wish = right * axis.x + forward * axis.y;

    let input_scale = control.movement_input_scale();
    input.set_wish(world_wish, axis.length() * input_scale);
    input.set_speed_multiplier(speed_multiplier);
    let jump_enabled = input_scale >= 0.5;
    input.jump_held = jump_enabled && keyboard.pressed(KeyCode::Space);
    input.jump_pressed |= jump_enabled && keyboard.just_pressed(KeyCode::Space);
}

pub fn noclip_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            &mut Transform,
            &CharacterLocomotionFrame,
            &CharacterControlFrame,
            &PlayerAim,
            &PlayerController,
            &PlayerNoclip,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    let (mut body, frame, control, aim, controller, noclip, mut velocity) =
        player.into_inner();

    if !noclip.active || gameplay_suppressed(&keyboard, &capture) {
        return;
    }

    let horizontal = keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8;
    let forward = keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    let vertical =
        keyboard.pressed(KeyCode::Space) as i8 - keyboard.pressed(KeyCode::ControlLeft) as i8;

    let view_rotation = control.rotation() * aim.local_rotation();
    let physical_up = frame.up();
    let mut wish = view_rotation * Vec3::X * horizontal as f32
        + view_rotation * Vec3::NEG_Z * forward as f32
        + physical_up * vertical as f32;

    wish = wish.normalize_or_zero();

    let boost = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        controller.sprint_multiplier
    } else {
        1.0
    };

    body.translation += wish * controller.noclip_speed.max(0.0) * boost * time.delta_secs();
    velocity.0 = Vec3::ZERO;
}
