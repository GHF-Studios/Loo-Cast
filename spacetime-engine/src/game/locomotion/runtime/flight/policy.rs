//! Resolve one flight velocity from intent, motion, and navigation policy.

use super::cruise::{CruiseStep, step_cruise};
use crate::game::locomotion::{
    FlightActuation, FlightAttitudeCommand, FlightControlIntent, LocomotionCapabilities,
    MotionKernel,
};
use crate::game::navigation::{
    AdaptiveCruise, TravelAssistance, TravelAssistanceState, TravelEnvelope, TravelProfile,
    TravelState,
};
use crate::spatial::UsfCanonicalMotion;
use bevy::{math::DVec3, prelude::*};

pub(super) fn vec3_to_dvec3(value: Vec3) -> DVec3 {
    DVec3::new(f64::from(value.x), f64::from(value.y), f64::from(value.z))
}

fn flight_wish(intent: &FlightControlIntent, attitude: Quat) -> DVec3 {
    let axes = intent.translation_axes();
    vec3_to_dvec3(
        (attitude * Vec3::X * axes.x
            + attitude * Vec3::NEG_Z * axes.z
            + attitude * Vec3::Y * axes.y)
            .normalize_or_zero(),
    )
}

pub(super) fn integrate_flight_attitude(
    current: Quat,
    command: FlightAttitudeCommand,
    profile: &TravelProfile,
    dt_seconds: f32,
) -> Quat {
    let current = current.normalize();
    let dt = dt_seconds.max(0.0);

    match command {
        FlightAttitudeCommand::Hold => current,
        FlightAttitudeCommand::AngularVelocityLocal(requested) => {
            let limits = Vec3::new(
                profile.flight.pitch_rate_radians_per_second.max(0.0),
                profile.flight.yaw_rate_radians_per_second.max(0.0),
                profile.flight.roll_rate_radians_per_second.max(0.0),
            );
            let rate = requested.clamp(-limits, limits);
            let delta = Quat::from_rotation_x(rate.x * dt)
                * Quat::from_rotation_y(rate.y * dt)
                * Quat::from_rotation_z(rate.z * dt);
            (current * delta).normalize()
        }
        FlightAttitudeCommand::TargetOrientation(target) => {
            let mut target = target.normalize();
            if current.dot(target) < 0.0 {
                target = -target;
            }

            let angle = 2.0 * current.dot(target).clamp(-1.0, 1.0).acos();
            let maximum_step = profile
                .flight
                .target_attitude_response_radians_per_second
                .max(0.0)
                * dt;
            if angle <= maximum_step || angle <= 1.0e-6 {
                target
            } else {
                current
                    .slerp(target, (maximum_step / angle).clamp(0.0, 1.0))
                    .normalize()
            }
        }
    }
}

fn integrate_local_inertial_velocity(
    current_velocity: DVec3,
    wish: DVec3,
    gravity_acceleration: DVec3,
    dt_seconds: f64,
    thrust_acceleration: f64,
    rcs_braking_acceleration: f64,
    thrusters_enabled: bool,
    rcs_enabled: bool,
) -> DVec3 {
    if dt_seconds <= 0.0 {
        return current_velocity;
    }

    let thrust_acceleration = thrust_acceleration.max(0.0);
    let thrusting =
        thrusters_enabled && thrust_acceleration > 0.0 && wish.length_squared() > 1.0e-18;

    let mut next_velocity = current_velocity + gravity_acceleration * dt_seconds;
    if thrusting {
        next_velocity += wish * thrust_acceleration * dt_seconds;
    }

    // RCS damping is a bounded braking acceleration, not a velocity reset.
    // Do not fight an active main-thruster command; braking takes over when
    // translational thrust is absent/released.
    if rcs_enabled && !thrusting {
        let speed = next_velocity.length();
        let delta_speed = rcs_braking_acceleration.max(0.0) * dt_seconds;
        if speed > 0.0 && speed <= delta_speed {
            DVec3::ZERO
        } else if speed > 0.0 {
            next_velocity * ((speed - delta_speed) / speed)
        } else {
            next_velocity
        }
    } else {
        next_velocity
    }
}

fn boost_multiplier(intent: &FlightControlIntent, profile: &TravelProfile) -> f64 {
    if intent.boost() {
        f64::from(profile.flight.boost_multiplier.max(0.0))
    } else {
        1.0
    }
}

/// Flight policy consumes canonical motion and intent. Chart projection and
/// collision remain the caller's responsibility after this step.
pub(super) struct FlightVelocityStep<'a> {
    pub(super) kernel: MotionKernel,
    pub(super) actuation: &'a FlightActuation,
    pub(super) capabilities: &'a LocomotionCapabilities,
    pub(super) intent: &'a FlightControlIntent,
    pub(super) profile: &'a TravelProfile,
    pub(super) envelope: &'a TravelEnvelope,
    pub(super) travel: &'a TravelState,
    pub(super) assistance: &'a TravelAssistanceState,
    pub(super) cruise: &'a mut AdaptiveCruise,
    pub(super) motion: &'a UsfCanonicalMotion,
    pub(super) rotation: Quat,
    pub(super) gravity: DVec3,
    pub(super) delta_seconds: f64,
    pub(super) delta_seconds_f32: f32,
}

pub(super) fn step_flight_velocity(step: FlightVelocityStep<'_>) -> DVec3 {
    let FlightVelocityStep {
        kernel,
        actuation,
        capabilities,
        intent,
        profile,
        envelope,
        travel,
        assistance,
        cruise,
        motion,
        rotation,
        gravity,
        delta_seconds,
        delta_seconds_f32,
    } = step;
    let dt = delta_seconds;
    let wish = flight_wish(intent, rotation);
    let pace = f64::from(intent.pace_multiplier().max(0.0));
    let boost = boost_multiplier(intent, profile);

    if assistance.mode() != TravelAssistance::Cruise {
        cruise.was_active = false;
    } else {
        let just_engaged = !cruise.was_active;
        cruise.was_active = true;
        return step_cruise(CruiseStep {
            state: cruise,
            envelope,
            travel,
            profile,
            intent,
            current_velocity: motion.velocity_metres_per_second(),
            current_speed: motion.speed_metres_per_second(),
            rotation,
            pace,
            delta_seconds: delta_seconds_f32,
            just_engaged,
        });
    }

    match kernel {
        MotionKernel::InertialFlight => integrate_local_inertial_velocity(
            motion.velocity_metres_per_second(),
            wish,
            gravity,
            dt,
            f64::from(
                profile
                    .flight
                    .local_acceleration_metres_per_second2
                    .max(0.0),
            ) * pace
                * boost,
            f64::from(profile.flight.rcs_braking_acceleration_metres_per_second2),
            capabilities.main_propulsion() && actuation.thrusters_enabled(),
            capabilities.reaction_control() && actuation.rcs_enabled(),
        ),
        MotionKernel::Character | MotionKernel::Disabled => unreachable!(),
    }
}
