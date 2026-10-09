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
    DeveloperMotionOverride, FlightActuation, FlightAttitudeCommand, FlightControlIntent,
    FlightThrottle, MotionAuthorityReason, MotionExecution, MotionKernel,
};
use crate::{
    ecs::UsfOwnershipQuery,
    game::{
        control::LocalControlSubject,
        flight::{FlightCapabilities, FlightSafetyProfile},
        navigation::{
            AdaptiveCruise, TravelAssistanceState, TravelEnvelope, TravelProfile, TravelState,
        },
    },
    physics::{
        collision_query::{
            UsfCanonicalSweep, UsfCollisionQueryDemand, UsfCollisionQueryFrame, UsfProposedSweeps,
            UsfSweepResolution,
        },
        gravity::GravitySample,
        slice::UsfPhysicsSliceQuery,
        topology::KinematicQueryExclusions,
    },
    spatial::{
        UsfCanonicalMotion, UsfMotionAuthority, UsfPosition, UsfRuntimeChartState, UsfScaleLayer,
        UsfSpatialTransitions, runtime_step_is_representable,
    },
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
use policy::{
    FlightVelocityStep, capture_cruise_exit_velocity, integrate_flight_attitude,
    step_flight_velocity,
};

#[derive(Debug, Clone, Copy)]
struct PreparedFlightStep {
    entity: Entity,
    semantic: Entity,
    dt_seconds: f64,
    desired_velocity: bevy::math::DVec3,
    collision_required: bool,
}

#[derive(Resource, Default)]
pub(in crate::game::locomotion) struct PreparedFlightMotion(Option<PreparedFlightStep>);

/// Physical attitude is resolved before translational velocity so propulsion
/// uses the orientation committed for this fixed step.
fn update_flight_attitude(
    intent: &FlightControlIntent,
    actuation: &FlightActuation,
    profile: &TravelProfile,
    body: &mut Transform,
    motion: &mut UsfCanonicalMotion,
    delta_seconds_f32: f32,
) {
    let command = if intent.active() {
        intent.attitude()
    } else {
        FlightAttitudeCommand::Hold
    };
    let (rotation, angular_velocity) = integrate_flight_attitude(
        body.rotation,
        motion.angular_velocity_radians_per_second(),
        command,
        actuation.angular_assist_enabled(),
        profile,
        delta_seconds_f32,
    );
    body.rotation = rotation;
    motion.set_angular_velocity_radians_per_second(angular_velocity);
}

pub(in crate::game::locomotion) fn prepare_flight_movement(
    time: Res<Time<Fixed>>,
    mut prepared: ResMut<PreparedFlightMotion>,
    mut proposed_sweeps: ResMut<UsfProposedSweeps>,
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &mut MotionExecution,
            &FlightActuation,
            &mut UsfCanonicalMotion,
        ),
        With<LocalControlSubject>,
    >,
    mut policy: Query<(
        &FlightControlIntent,
        &TravelProfile,
        &TravelEnvelope,
        &TravelState,
        &mut TravelAssistanceState,
        &FlightCapabilities,
        &FlightSafetyProfile,
        &GravitySample,
        Option<&DeveloperMotionOverride>,
        &mut AdaptiveCruise,
        &mut FlightThrottle,
        Option<&UsfCollisionQueryDemand>,
    )>,
) {
    prepared.0 = None;
    let (entity, mut body, layer, mut execution, actuation, mut motion) = subject.into_inner();

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
        mut assistance,
        capabilities,
        safety_profile,
        gravity,
        developer_motion,
        mut cruise,
        mut throttle,
        collision_demand,
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
        actuation,
        profile,
        &mut body,
        &mut motion,
        time.delta_secs(),
    );

    let debug_traversal = developer_motion.is_some_and(|state| state.characteristic_traversal());
    if !debug_traversal {
        if assistance.emergency_capture_pending() {
            throttle.set(0.0);
        } else if intent.active() {
            throttle.advance(intent.throttle_axis(), time.delta_secs());
        }
        if assistance.mode() == crate::game::navigation::TravelAssistance::Cruise
            && throttle.value() < 0.0
        {
            throttle.set(0.0);
        }
    }

    let next_velocity = step_flight_velocity(FlightVelocityStep {
        kernel,
        actuation,
        capabilities,
        intent,
        throttle: throttle.value(),
        characteristic_traversal: debug_traversal,
        profile,
        envelope,
        travel,
        assistance: &assistance,
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

    let next_velocity = if assistance.capture_pending() && debug_traversal {
        assistance.finish_capture();
        next_velocity
    } else if assistance.capture_pending() {
        let previous_speed = next_velocity.length();
        let limit = if assistance.emergency_capture_pending() {
            safety_profile.emergency_contact_speed_metres_per_second()
        } else {
            profile.manual.maximum_metres_per_second
        };
        let captured = capture_cruise_exit_velocity(next_velocity, limit);
        info!(
            subject = ?entity,
            emergency = assistance.emergency_capture_pending(),
            previous_speed_metres_per_second = previous_speed,
            captured_speed_metres_per_second = captured.length(),
            "Lattice Drive dropout motion capture"
        );
        assistance.finish_capture();
        captured
    } else {
        next_velocity
    };
    let proposed_displacement = next_velocity * dt;
    if !next_velocity.is_finite() || !proposed_displacement.is_finite() {
        error!(subject = ?entity, ?next_velocity, dt_seconds = dt, "flight policy produced an unrepresentable canonical step; stopping subject");
        throttle.set(0.0);
        cruise.was_active = false;
        prepared.0 = Some(PreparedFlightStep {
            entity,
            semantic: semantic_entity,
            dt_seconds: dt,
            desired_velocity: bevy::math::DVec3::ZERO,
            collision_required: false,
        });
        return;
    }
    if execution.kernel() == MotionKernel::InertialFlight
        && !motion.is_canonical_kinematic()
        && !runtime_step_is_representable(body.translation, next_velocity, layer.scale(), dt)
    {
        motion.set_authority(UsfMotionAuthority::CanonicalKinematics);
        let kernel = execution.kernel();
        let collision_policy = execution.collision_policy();
        let velocity_semantics = execution.velocity_semantics();
        execution.resolve(
            kernel,
            collision_policy,
            velocity_semantics,
            MotionAuthorityReason::NumericalRange,
        );
    }
    let collision_required = motion.is_canonical_kinematic()
        && !developer_motion.is_some_and(|override_| override_.ignore_collision());
    if collision_required && next_velocity.length_squared() > f64::EPSILON {
        if let (Some(demand), Ok(&start)) =
            (collision_demand, semantic_positions.get(semantic_entity))
        {
            proposed_sweeps.submit(
                UsfCanonicalSweep::new(
                    semantic_entity,
                    start,
                    proposed_displacement,
                    dt,
                    demand.bounding_radius_metres(),
                ),
                demand.target_error_metres(),
            );
        }
    }
    prepared.0 = Some(PreparedFlightStep {
        entity,
        semantic: semantic_entity,
        dt_seconds: dt,
        desired_velocity: next_velocity,
        collision_required,
    });
}

/// Resolve the prepared step only after canonical providers have answered its
/// proposed sweep. This system alone commits flight position and velocity.
pub(in crate::game::locomotion) fn flight_movement(
    time: Res<Time<Fixed>>,
    frame: Res<UsfRuntimeChartState>,
    mut transitions: ResMut<UsfSpatialTransitions>,
    mut prepared: ResMut<PreparedFlightMotion>,
    collision_frame: Res<UsfCollisionQueryFrame>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsSliceQuery,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &MotionExecution,
            &mut UsfCanonicalMotion,
            &mut LinearVelocity,
            Option<&Collider>,
            Option<&KinematicQueryExclusions>,
        ),
        With<LocalControlSubject>,
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let Some(step) = prepared.0.take() else {
        return;
    };
    let (entity, mut body, layer, execution, mut motion, mut linear_velocity, collider, exclusions) =
        subject.into_inner();
    if entity != step.entity {
        error!(prepared = ?step.entity, controlled = ?entity, "flight subject changed during fixed-step motion transaction");
        return;
    }

    if motion.is_canonical_kinematic() {
        let resolution =
            if step.collision_required && step.desired_velocity.length_squared() > f64::EPSILON {
                collision_frame
                    .request_for_subject(step.semantic)
                    .and_then(|request| collision_frame.resolution_for(request.id()))
                    .unwrap_or(UsfSweepResolution::Unknown { safe_fraction: 0.0 })
            } else {
                UsfSweepResolution::Clear
            };
        let fraction = resolution.safe_fraction().clamp(0.0, 1.0);
        let displacement = step.desired_velocity * step.dt_seconds * fraction;
        let accepted_velocity = match resolution {
            UsfSweepResolution::Clear => step.desired_velocity,
            UsfSweepResolution::Contact { normal, .. }
                if normal.is_finite() && normal.length_squared() > 0.5 =>
            {
                warn!(subject = ?step.semantic, ?resolution, "canonical flight motion clipped by collision query");
                let inward_speed = step.desired_velocity.dot(normal).min(0.0);
                step.desired_velocity - normal * inward_speed
            }
            UsfSweepResolution::Contact { .. } | UsfSweepResolution::Unknown { .. } => {
                warn!(subject = ?step.semantic, ?resolution, "canonical flight motion clipped by collision query");
                bevy::math::DVec3::ZERO
            }
        };
        let accepted_velocity = if accepted_velocity.is_finite() {
            accepted_velocity
        } else {
            error!(subject = ?step.semantic, ?accepted_velocity, "collision response produced invalid velocity; stopping subject");
            bevy::math::DVec3::ZERO
        };
        if !commit_canonical_motion(
            displacement,
            &frame,
            step.semantic,
            layer.scale(),
            &mut body,
            &mut linear_velocity,
            &mut motion,
            accepted_velocity,
            &mut semantic_positions,
            &mut transitions,
        ) {
            warn!(subject = ?step.semantic, "canonical flight transaction rejected; preserving prior semantic motion");
        }
        return;
    }

    motion.set_velocity_metres_per_second(step.desired_velocity);
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
    commit_runtime_position_to_canonical(
        &frame,
        step.semantic,
        layer.scale(),
        &body,
        &mut semantic_positions,
    );
    debug_assert!(
        matches!(execution.kernel(), MotionKernel::InertialFlight),
        "runtime-authoritative flight must use a collision-capable local motion kernel"
    );
}
