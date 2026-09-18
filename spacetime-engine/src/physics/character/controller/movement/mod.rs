//! Character velocity integration and grounded-state transitions.

use bevy::prelude::*;

use super::super::{
    CharacterGroundState, CharacterMovementConfig, CharacterMovementInput, accelerate,
    air_accelerate, apply_friction, reject,
};

pub(super) fn reset_transition_flags(ground: &mut CharacterGroundState) {
    ground.just_landed = false;
    ground.just_left_ground = false;
    ground.just_jumped = false;
}

/// Integrate player intent and the first half of airborne gravity.
///
/// Ground classification must already be current for this tick. Jumping mutates
/// that classification immediately so subsequent route selection cannot treat a
/// jump as grounded movement.
pub(super) fn integrate_pre_move_velocity(
    config: &CharacterMovementConfig,
    input: &CharacterMovementInput,
    ground: &mut CharacterGroundState,
    up: Vec3,
    dt: f32,
    velocity: &mut Vec3,
) {
    if ground.grounded && velocity.dot(up) < 0.0 {
        *velocity = reject(*velocity, up);
    }

    let planar_wish = reject(input.wish_direction, up);
    let wish_dir = planar_wish.normalize_or_zero();
    let wish_speed = config.max_ground_speed
        * input.wish_speed_fraction.clamp(0.0, 1.0)
        * input.speed_multiplier.max(0.0);

    let wants_jump = if config.auto_bhop {
        input.jump_held || input.jump_pressed
    } else {
        input.jump_pressed
    };

    if ground.grounded && wants_jump {
        *velocity = reject(*velocity, up) + up * config.jump_speed;
        ground.grounded = false;
        ground.ground_entity = None;
        ground.just_jumped = true;
        ground.just_left_ground = true;
    }

    if ground.grounded {
        let planar = apply_friction(
            reject(*velocity, up),
            config.friction,
            config.stop_speed,
            config.surface_friction,
            dt,
        );

        *velocity = accelerate(
            planar,
            wish_dir,
            wish_speed,
            config.ground_acceleration,
            config.surface_friction,
            dt,
        );
    } else {
        *velocity = air_accelerate(
            *velocity,
            wish_dir,
            wish_speed,
            config.air_wish_speed_cap,
            config.air_acceleration,
            config.surface_friction,
            dt,
        );

        // Split gravity: half before movement, half after.
        *velocity -= up * (config.gravity * dt * 0.5);
    }
}

pub(super) fn apply_post_move_gravity(
    config: &CharacterMovementConfig,
    grounded: bool,
    up: Vec3,
    dt: f32,
    velocity: &mut Vec3,
) {
    if !grounded {
        *velocity -= up * (config.gravity * dt * 0.5);
    }
}

pub(super) fn finish_transition_flags(
    ground: &mut CharacterGroundState,
    was_grounded: bool,
) {
    if !was_grounded && ground.grounded {
        ground.just_landed = true;
    }
    if was_grounded && !ground.grounded && !ground.just_jumped {
        ground.just_left_ground = true;
    }
}
