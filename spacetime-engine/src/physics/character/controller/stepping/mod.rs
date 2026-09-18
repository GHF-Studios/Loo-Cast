//! Collision-constrained movement route selection.
//!
//! Direct sliding is always evaluated. Grounded movement may additionally try a
//! raised route; that route wins only when it produces strictly more planar
//! progress than the direct route.

use std::time::Duration;

use avian3d::character_controller::move_and_slide::{
    MoveAndSlideHitResponse, MoveAndSlideOutput,
};
use bevy::prelude::*;

use super::CollisionContext;
use super::grounding::probe_ground;
use super::super::{CharacterMovementConfig, reject};

pub(super) fn move_with_step_selection(
    collision: &CollisionContext<'_>,
    start: Vec3,
    velocity: Vec3,
    duration: Duration,
    up: Vec3,
    moving_on_ground: bool,
    config: &CharacterMovementConfig,
) -> MoveAndSlideOutput {
    let direct = slide(collision, start, velocity, duration);

    if !moving_on_ground || config.step_height <= 0.0 {
        return direct;
    }

    step_route(collision, start, velocity, duration, up, config)
        .filter(|stepped| {
            let direct_planar = reject(direct.position - start, up).length_squared();
            let step_planar = reject(stepped.position - start, up).length_squared();
            step_planar > direct_planar + 1.0e-8
        })
        .unwrap_or(direct)
}

fn slide(
    collision: &CollisionContext<'_>,
    position: Vec3,
    velocity: Vec3,
    duration: Duration,
) -> MoveAndSlideOutput {
    collision.move_and_slide.move_and_slide(
        collision.collider,
        position,
        collision.rotation,
        velocity,
        duration,
        collision.move_config,
        collision.filter,
        |_| MoveAndSlideHitResponse::Accept,
    )
}

fn step_route(
    collision: &CollisionContext<'_>,
    start: Vec3,
    velocity: Vec3,
    duration: Duration,
    up: Vec3,
    config: &CharacterMovementConfig,
) -> Option<MoveAndSlideOutput> {
    let up_movement = up * config.step_height;
    let raised_distance = match collision.move_and_slide.cast_move(
        collision.collider,
        start,
        collision.rotation,
        up_movement,
        collision.move_config.skin_width,
        collision.filter,
    ) {
        Some(hit) => hit.distance,
        None => config.step_height,
    };

    if raised_distance <= collision.move_config.skin_width {
        return None;
    }

    let raised = start + up * raised_distance;
    let mut moved = slide(collision, raised, velocity, duration);

    let hit = probe_ground(
        collision,
        moved.position,
        up,
        config.step_height + config.ground_snap_distance,
        config.min_ground_dot,
    )?;

    moved.position -= up * hit.distance;
    moved.projected_velocity = reject(moved.projected_velocity, up);
    Some(moved)
}
