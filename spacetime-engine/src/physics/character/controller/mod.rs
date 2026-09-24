//! Character-motor orchestration.
//!
//! This module owns the fixed-tick character movement sequence. Focused child
//! modules own contact classification, velocity integration, route selection,
//! and dynamic-body interaction. The ECS system itself only acquires state,
//! delegates one-character simulation, and publishes push messages.

use std::time::Duration;

use avian3d::{
    character_controller::move_and_slide::{MoveAndSlide, MoveAndSlideConfig},
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    physics::{
        chart::UsfPhysicsCharts,
        gravity::GravitySample,
        topology::KinematicQueryExclusions,
    },
    spatial::UsfScaleLayer,
};

use super::{
    CharacterGroundState, CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig,
    CharacterMovementInput, ResolvedCharacterMovementConfig,
};

mod grounding;
mod movement;
mod pushing;
mod stepping;

pub(crate) use pushing::{
    CharacterPush, MAX_DYNAMIC_CONTACT_DELTA_SPEED, dynamic_contact_delta_velocity,
};
pub(super) use pushing::{apply_character_pushes, receive_dynamic_contact_pushes};

/// Collision/query state shared by one character motor tick.
///
/// This deliberately contains only immutable query mechanism. Character
/// semantic state remains explicit in [`MotorTick`].
struct CollisionContext<'a, 'w, 's> {
    move_and_slide: &'a MoveAndSlide<'w, 's>,
    collider: &'a Collider,
    rotation: Quat,
    move_config: &'a MoveAndSlideConfig,
    filter: &'a SpatialQueryFilter,
}

/// Mutable semantic state for one character during one fixed tick.
struct MotorTick<'a> {
    config: &'a ResolvedCharacterMovementConfig,
    input: &'a mut CharacterMovementInput,
    ground: &'a mut CharacterGroundState,
    velocity: &'a mut LinearVelocity,
    transform: &'a mut Transform,
    up: Vec3,
}

pub(super) fn simulate_character_motors(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsCharts,
    mut pushes: MessageWriter<CharacterPush>,
    mut query: Query<
        (
            Entity,
            &UsfScaleLayer,
            &Collider,
            &CharacterMovementConfig,
            &GravitySample,
            &CharacterLocomotionFrame,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
            &mut Transform,
            Option<&KinematicQueryExclusions>,
        ),
        With<CharacterMotor>,
    >,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let duration = time.delta();
    let move_config = MoveAndSlideConfig::default();

    for (
        entity,
        layer,
        collider,
        config,
        gravity,
        frame,
        mut input,
        mut ground,
        mut velocity,
        mut transform,
        exclusions,
    ) in &mut query
    {
        let excluded = std::iter::once(entity)
            .chain(exclusions.into_iter().flat_map(|exclusions| exclusions.iter()));
        let filter = physics_charts.filter_for_scale(layer.scale(), excluded);

        let resolved_config =
            config.resolve_for_chart(layer.scale(), gravity.acceleration_metres_per_second2());

        let collision = CollisionContext {
            move_and_slide: &move_and_slide,
            collider,
            rotation: transform.rotation,
            move_config: &move_config,
            filter: &filter,
        };
        let tick = MotorTick {
            config: &resolved_config,
            input: &mut input,
            ground: &mut ground,
            velocity: &mut velocity,
            transform: &mut transform,
            up: frame.up(),
        };

        if let Some(push) = simulate_character_motor(&collision, tick, dt, duration) {
            pushes.write(push);
        }
    }
}

fn simulate_character_motor(
    collision: &CollisionContext<'_, '_, '_>,
    tick: MotorTick<'_>,
    dt: f32,
    duration: Duration,
) -> Option<CharacterPush> {
    movement::reset_transition_flags(tick.ground);
    grounding::reconcile_penetration(collision, tick.transform, tick.velocity);

    let was_grounded = tick.ground.is_grounded();
    grounding::refresh_ground_state(
        collision,
        tick.transform.translation,
        tick.up,
        tick.config,
        tick.ground,
    );

    movement::integrate_pre_move_velocity(
        tick.config,
        tick.input,
        tick.ground,
        tick.up,
        dt,
        &mut tick.velocity.0,
    );

    let start = tick.transform.translation;
    let moving_from_ground = tick.ground.is_grounded();
    let moving_on_ground =
        moving_from_ground && super::reject(tick.velocity.0, tick.up).length_squared() > 1.0e-8;

    let push =
        pushing::detect_outgoing_push(collision, start, tick.velocity.0, dt);

    let chosen = stepping::move_with_step_selection(
        collision,
        start,
        tick.velocity.0,
        duration,
        tick.up,
        moving_on_ground,
        tick.config,
    );

    tick.transform.translation = chosen.position;
    tick.velocity.0 = chosen.projected_velocity;

    movement::apply_post_move_gravity(
        tick.config,
        tick.ground.is_grounded(),
        dt,
        &mut tick.velocity.0,
    );

    grounding::finalize_grounding(
        collision,
        tick.transform,
        tick.velocity,
        tick.ground,
        tick.up,
        moving_from_ground,
        tick.config,
    );

    movement::finish_transition_flags(tick.ground, was_grounded);

    // One-shot input is consumed by the physics tick; held input persists.
    tick.input.jump_pressed = false;

    push
}
