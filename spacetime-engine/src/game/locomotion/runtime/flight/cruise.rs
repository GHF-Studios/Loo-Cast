//! Canonical-SI cruise throttle and directional policy.

use super::policy::vec3_to_dvec3;
use crate::game::navigation::{AdaptiveCruise, TravelEnvelope, TravelProfile, TravelState};
use bevy::{math::DVec3, prelude::*};

/// One canonical-SI cruise policy update; the caller owns chart projection
/// and the final semantic motion commit.
pub(super) struct CruiseStep<'a> {
    pub(super) state: &'a mut AdaptiveCruise,
    pub(super) envelope: &'a TravelEnvelope,
    pub(super) travel: &'a TravelState,
    pub(super) profile: &'a TravelProfile,
    pub(super) throttle: f32,
    pub(super) current_velocity: DVec3,
    pub(super) current_speed: f64,
    pub(super) rotation: Quat,
    pub(super) pace: f64,
    pub(super) delta_seconds: f32,
    pub(super) just_engaged: bool,
}

pub(super) fn step_cruise(step: CruiseStep<'_>) -> DVec3 {
    let CruiseStep {
        state,
        envelope,
        travel,
        profile,
        throttle,
        current_velocity,
        current_speed,
        rotation,
        pace,
        delta_seconds,
        just_engaged,
    } = step;
    state.speed_cap_metres_per_second = envelope.cruise_max_speed_metres_per_second;
    state.default_speed_metres_per_second = envelope.cruise_default_speed_metres_per_second;
    state.nearest_hard_clearance_metres = travel.nearest_body_clearance_metres;
    state.medium_speed_cap_metres_per_second = envelope.medium_speed_cap_metres_per_second;

    if just_engaged {
        state.throttle = throttle.clamp(0.0, 1.0);
        state.speed_metres_per_second = current_speed;
        return current_velocity;
    }

    state.throttle = throttle.clamp(0.0, 1.0);

    // Pace acts beneath the navigation envelope; hard-body and medium limits
    // remain authoritative even when the controller requests a higher speed.
    let requested =
        (envelope.cruise_max_speed_metres_per_second * f64::from(state.throttle.powf(2.0)) * pace)
            .clamp(0.0, envelope.cruise_max_speed_metres_per_second);
    state.speed_metres_per_second = smooth_log_value(
        state.speed_metres_per_second,
        requested,
        delta_seconds,
        profile.cruise.speed_response,
    );

    let direction = vec3_to_dvec3(rotation * Vec3::NEG_Z).normalize_or_zero();
    cruise_velocity(current_velocity, direction, state.speed_metres_per_second)
}

fn cruise_velocity(
    current: DVec3,
    desired_direction: DVec3,
    commanded_speed_metres_per_second: f64,
) -> DVec3 {
    let speed = commanded_speed_metres_per_second.max(0.0);
    if speed <= f64::EPSILON {
        return DVec3::ZERO;
    }

    let direction = desired_direction.normalize_or_zero();
    if direction == DVec3::ZERO {
        let current_direction = current.normalize_or_zero();
        return current_direction * speed;
    }

    // Preserve some directional inertia while steering, but NEVER preserve it
    // as extra magnitude. The combined steering vector is normalized back to
    // the commanded scalar speed, so merely rotating the look basis cannot
    // accelerate or decelerate the craft.
    let lateral = current - direction * current.dot(direction);
    let steering = lateral + direction * speed;
    let resolved_direction = steering.normalize_or_zero();

    if resolved_direction == DVec3::ZERO {
        direction * speed
    } else {
        resolved_direction * speed
    }
}

fn smooth_log_value(current: f64, target: f64, dt: f32, response: f64) -> f64 {
    let current_log = (1.0 + current.max(0.0)).log10();
    let target_log = (1.0 + target.max(0.0)).log10();
    let alpha = 1.0 - (-response * f64::from(dt)).exp();
    10.0_f64.powf(current_log + (target_log - current_log) * alpha) - 1.0
}
