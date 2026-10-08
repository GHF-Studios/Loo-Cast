//! Controlled flight orchestration: policy, collision, and semantic commit.
//!
//! ## Module map
//!
//! - `commit`: Commit resolved flight motion across the runtime chart and semantic USF boundary.
//! - `cruise`: Canonical-SI cruise throttle and directional policy.
//! - `policy`: Resolve one flight velocity from intent, motion, and navigation policy.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

use super::super::{
    DeveloperMotionOverride, FlightActuation, FlightControlIntent, MotionExecution, MotionKernel,
};
use crate::{
    ecs::UsfOwnershipQuery,
    game::{
        control::LocalControlSubject,
        flight::FlightCapabilities,
        navigation::{
            AdaptiveCruise, TravelAssistanceState, TravelEnvelope, TravelProfile, TravelState,
        },
    },
    physics::{
        gravity::GravitySample, slice::UsfPhysicsSliceQuery, topology::KinematicQueryExclusions,
    },
    spatial::{UsfCanonicalMotion, UsfPosition, UsfRuntimeChartState, UsfScaleLayer},
};
use avian3d::{
    character_controller::move_and_slide::MoveAndSlide,
    prelude::{Collider, LinearVelocity},
};
use bevy::prelude::*;

mod commit;
mod cruise;
mod policy;

use commit::{
    collide_runtime_motion, commit_canonical_motion, commit_runtime_position_to_canonical,
};
use policy::{FlightVelocityStep, integrate_flight_attitude, step_flight_velocity};

/// Physical attitude is resolved before translational velocity so propulsion
/// uses the orientation committed for this fixed step.
fn update_flight_attitude(
    intent: &FlightControlIntent,
    profile: &TravelProfile,
    body: &mut Transform,
    motion: &mut UsfCanonicalMotion,
    delta_seconds: f64,
    delta_seconds_f32: f32,
) {
    if !intent.active() {
        motion.set_angular_velocity_radians_per_second(bevy::math::DVec3::ZERO);
        return;
    }

    let previous_rotation = body.rotation;
    body.rotation =
        integrate_flight_attitude(body.rotation, intent.attitude(), profile, delta_seconds_f32);
    let mut delta = (body.rotation * previous_rotation.conjugate()).normalize();
    if delta.w < 0.0 {
        delta = -delta;
    }
    let (axis, angle) = delta.to_axis_angle();
    let angular_velocity = if angle.is_finite() && angle > 1.0e-6 {
        let rate = f64::from(angle) / delta_seconds;
        bevy::math::DVec3::new(
            f64::from(axis.x) * rate,
            f64::from(axis.y) * rate,
            f64::from(axis.z) * rate,
        )
    } else {
        bevy::math::DVec3::ZERO
    };
    motion.set_angular_velocity_radians_per_second(angular_velocity);
}

pub(in crate::game::locomotion) fn flight_movement(
    time: Res<Time<Fixed>>,
    frame: Res<UsfRuntimeChartState>,
    ownership: UsfOwnershipQuery,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsSliceQuery,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &MotionExecution,
            &FlightActuation,
            &mut UsfCanonicalMotion,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
    mut policy: Query<(
        &FlightControlIntent,
        &TravelProfile,
        &TravelEnvelope,
        &TravelState,
        &TravelAssistanceState,
        &FlightCapabilities,
        &GravitySample,
        Option<&DeveloperMotionOverride>,
        &mut AdaptiveCruise,
        Option<&Collider>,
        Option<&KinematicQueryExclusions>,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let (entity, mut body, layer, execution, actuation, mut motion, mut linear_velocity) =
        subject.into_inner();

    let Some(semantic_entity) = ownership.semantic_of(entity) else {
        error!(
            realization = ?entity,
            "controlled locomotion subject has no semantic USF owner"
        );
        return;
    };

    let Ok((
        intent,
        profile,
        envelope,
        travel,
        assistance,
        capabilities,
        gravity,
        developer_motion,
        mut cruise,
        collider,
        exclusions,
    )) = policy.get_mut(entity)
    else {
        return;
    };

    let kernel = execution.kernel();
    if matches!(kernel, MotionKernel::Character | MotionKernel::Disabled) {
        cruise.was_active = false;
        return;
    }

    let dt = time.delta().as_secs_f64();
    if dt <= 0.0 {
        return;
    }

    update_flight_attitude(
        intent,
        profile,
        &mut body,
        &mut motion,
        dt,
        time.delta_secs(),
    );

    let next_velocity = step_flight_velocity(FlightVelocityStep {
        kernel,
        actuation,
        capabilities,
        intent,
        profile,
        envelope,
        travel,
        assistance,
        cruise: &mut cruise,
        motion: &motion,
        rotation: body.rotation,
        gravity: if developer_motion.is_some_and(|override_| override_.ignore_gravity()) {
            bevy::math::DVec3::ZERO
        } else {
            gravity.acceleration_metres_per_second2()
        },
        delta_seconds: dt,
        delta_seconds_f32: time.delta_secs(),
    });

    motion.set_velocity_metres_per_second(next_velocity);

    if motion.is_canonical_kinematic() {
        commit_canonical_motion(
            dt,
            &frame,
            semantic_entity,
            layer.scale(),
            &mut body,
            &mut linear_velocity,
            &motion,
            &mut semantic_positions,
        );
        return;
    }

    let desired_native_velocity = motion.native_velocity(layer.scale());
    let projected = collide_runtime_motion(
        entity,
        time.delta(),
        layer.scale(),
        &mut body,
        collider,
        exclusions,
        desired_native_velocity,
        &move_and_slide,
        &physics_charts,
    );
    linear_velocity.0 = projected;
    motion.set_from_native_velocity(layer.scale(), projected);

    // Runtime physics owns the collision solve for this branch, but canonical
    // USF position remains semantic authority. Persist the collision-resolved
    // runtime pose before spatial projection can reconstruct an older position.
    commit_runtime_position_to_canonical(
        &frame,
        semantic_entity,
        layer.scale(),
        &body,
        &mut semantic_positions,
    );

    debug_assert!(
        matches!(kernel, MotionKernel::InertialFlight),
        "runtime-authoritative flight must use a collision-capable local motion kernel"
    );
}
