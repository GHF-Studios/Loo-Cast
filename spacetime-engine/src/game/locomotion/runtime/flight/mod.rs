//! Controlled flight orchestration: policy, collision, and semantic commit.

use super::super::{ControlledSubjectLocomotion, FlightControlIntent, MotionKernel};
use crate::{
    ecs::UsfOwnershipQuery,
    game::{
        control::LocalControlSubject,
        navigation::{
            AdaptiveCruise, TravelAssistanceState, TravelEnvelope, TravelProfile, TravelState,
        },
    },
    physics::{
        character::CharacterLocomotionFrame, gravity::GravitySample, slice::UsfPhysicsSlices,
        topology::KinematicQueryExclusions,
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

pub(in crate::game::locomotion) fn flight_movement(
    time: Res<Time<Fixed>>,
    frame: Res<UsfRuntimeChartState>,
    ownership: UsfOwnershipQuery,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsSlices,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &ControlledSubjectLocomotion,
            &mut UsfCanonicalMotion,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
    mut policy: Query<(
        &CharacterLocomotionFrame,
        &FlightControlIntent,
        &TravelProfile,
        &TravelEnvelope,
        &TravelState,
        &TravelAssistanceState,
        &GravitySample,
        &mut AdaptiveCruise,
        Option<&Collider>,
        Option<&KinematicQueryExclusions>,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let (entity, mut body, layer, locomotion, mut motion, mut linear_velocity) =
        subject.into_inner();

    let Some(semantic_entity) = ownership.semantic_of(entity) else {
        error!(
            realization = ?entity,
            "controlled locomotion subject has no semantic USF owner"
        );
        return;
    };

    let Ok((
        locomotion_frame,
        intent,
        profile,
        envelope,
        travel,
        assistance,
        gravity,
        mut cruise,
        collider,
        exclusions,
    )) = policy.get_mut(entity)
    else {
        return;
    };

    let kernel = locomotion.kernel();
    if matches!(kernel, MotionKernel::Character | MotionKernel::Disabled) {
        cruise.was_active = false;
        return;
    }

    let dt = time.delta().as_secs_f64();
    if dt <= 0.0 {
        return;
    }

    if intent.active() {
        body.rotation =
            integrate_flight_attitude(body.rotation, intent.attitude(), profile, time.delta_secs());
    }

    let next_velocity = step_flight_velocity(FlightVelocityStep {
        kernel,
        locomotion,
        intent,
        profile,
        envelope,
        travel,
        assistance,
        cruise: &mut cruise,
        motion: &motion,
        rotation: body.rotation,
        up: locomotion_frame.up(),
        gravity: gravity.acceleration_metres_per_second2(),
        delta_seconds: dt,
        delta_seconds_f32: time.delta_secs(),
    });

    motion.set_velocity_metres_per_second(next_velocity);

    if motion.canonical_authority() {
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
        matches!(
            kernel,
            MotionKernel::InertialFlight
                | MotionKernel::ThrusterFlight
                | MotionKernel::ScaleNavigation
        ),
        "runtime-authoritative flight must use a collision-capable local motion kernel"
    );
}
