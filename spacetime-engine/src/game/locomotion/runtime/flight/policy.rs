//! Resolve one flight velocity from intent, motion, and navigation policy.

use super::cruise::{CruiseStep, step_cruise};
use crate::game::navigation::{
    AdaptiveCruise, TravelAssistance, TravelAssistanceState, TravelEnvelope, TravelProfile,
    TravelState,
};
use crate::game::{
    flight::FlightCapabilities,
    locomotion::{FlightActuation, FlightAttitudeCommand, FlightControlIntent, MotionKernel},
};
use crate::spatial::UsfCanonicalMotion;
use bevy::{math::DVec3, prelude::*};

pub(super) fn vec3_to_dvec3(value: Vec3) -> DVec3 {
    DVec3::new(f64::from(value.x), f64::from(value.y), f64::from(value.z))
}

fn flight_wish(intent: &FlightControlIntent, attitude: Quat, throttle: f32) -> DVec3 {
    let mut axes = intent.translation_axes();
    axes.z = throttle;
    vec3_to_dvec3(
        (attitude * Vec3::X * axes.x
            + attitude * Vec3::NEG_Z * axes.z
            + attitude * Vec3::Y * axes.y)
            .clamp_length_max(1.0),
    )
}

pub(super) fn capture_cruise_exit_velocity(velocity: DVec3, limit: f64) -> DVec3 {
    velocity.clamp_length_max(limit.max(0.0))
}

pub(super) fn integrate_flight_attitude(
    current: Quat,
    angular_velocity_world: DVec3,
    command: FlightAttitudeCommand,
    flight_assist: bool,
    profile: &TravelProfile,
    dt_seconds: f32,
) -> (Quat, DVec3) {
    let current = current.normalize();
    let dt = dt_seconds.max(0.0);
    let limits = Vec3::new(
        profile.flight.pitch_rate_radians_per_second.max(0.0),
        profile.flight.yaw_rate_radians_per_second.max(0.0),
        profile.flight.roll_rate_radians_per_second.max(0.0),
    );
    let maximum_rate = f64::from(limits.max_element());
    let desired = match command {
        FlightAttitudeCommand::Hold if !flight_assist => angular_velocity_world,
        FlightAttitudeCommand::Hold => DVec3::ZERO,
        FlightAttitudeCommand::AngularVelocityLocal(requested) => {
            let local = requested.clamp(-limits, limits);
            let world = current * local;
            DVec3::new(f64::from(world.x), f64::from(world.y), f64::from(world.z))
        }
        FlightAttitudeCommand::TargetOrientation(target) => {
            let mut delta = (target.normalize() * current.conjugate()).normalize();
            if delta.w < 0.0 {
                delta = -delta;
            }
            let (axis, angle) = delta.to_axis_angle();
            let rate = (f64::from(angle)
                * f64::from(
                    profile
                        .flight
                        .target_attitude_response_radians_per_second
                        .max(0.0),
                ))
            .min(maximum_rate);
            DVec3::new(f64::from(axis.x), f64::from(axis.y), f64::from(axis.z)) * rate
        }
    };
    let maximum_delta = maximum_rate * 4.0 * f64::from(dt);
    let next_angular_velocity =
        angular_velocity_world + (desired - angular_velocity_world).clamp_length_max(maximum_delta);
    let angle = next_angular_velocity.length() * f64::from(dt);
    if angle <= 1.0e-9 {
        return (current, next_angular_velocity);
    }
    let axis = next_angular_velocity.normalize();
    let delta = Quat::from_axis_angle(
        Vec3::new(axis.x as f32, axis.y as f32, axis.z as f32),
        angle as f32,
    );
    ((delta * current).normalize(), next_angular_velocity)
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
    pub(super) capabilities: &'a FlightCapabilities,
    pub(super) intent: &'a FlightControlIntent,
    pub(super) throttle: f32,
    pub(super) characteristic_traversal: bool,
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
        throttle,
        characteristic_traversal,
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
    let wish = flight_wish(intent, rotation, throttle);
    let pace = f64::from(intent.pace_multiplier().max(0.0));
    let boost = boost_multiplier(intent, profile);

    if characteristic_traversal {
        let direction = flight_wish(intent, rotation, intent.throttle_axis());
        return direction * envelope.manual_speed_metres_per_second * pace * boost;
    }

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
            throttle,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flight_assist_off_preserves_angular_momentum_on_release() {
        let profile = TravelProfile::spacecraft();
        let (rotation, angular_velocity) = integrate_flight_attitude(
            Quat::IDENTITY,
            DVec3::Z,
            FlightAttitudeCommand::Hold,
            false,
            &profile,
            0.5,
        );
        assert!(rotation != Quat::IDENTITY);
        assert!((angular_velocity.z - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn flight_assist_on_brakes_angular_momentum_on_release() {
        let profile = TravelProfile::spacecraft();
        let (_, angular_velocity) = integrate_flight_attitude(
            Quat::IDENTITY,
            DVec3::Z,
            FlightAttitudeCommand::Hold,
            true,
            &profile,
            0.5,
        );
        assert!(angular_velocity.length() < 1.0);
    }

    #[test]
    fn half_throttle_produces_half_forward_acceleration() {
        let intent = FlightControlIntent::default();
        let wish = flight_wish(&intent, Quat::IDENTITY, 0.5);
        let velocity = integrate_local_inertial_velocity(
            DVec3::ZERO,
            wish,
            DVec3::ZERO,
            1.0,
            10.0,
            0.0,
            true,
            false,
        );
        assert!((velocity.z + 5.0).abs() < 1.0e-6);
    }

    #[test]
    fn emergency_dropout_caps_extreme_cruise_speed_for_local_collision() {
        let captured = capture_cruise_exit_velocity(DVec3::X * 1.0e10, 100.0);
        assert!((captured.length() - 100.0).abs() < 1.0e-6);
        assert!(captured.x > 0.0);
    }
}
