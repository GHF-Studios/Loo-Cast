//! Subject safety policy and supervisor output.

use bevy::{math::DVec3, prelude::*};

/// Subject-owned flight safety policy.
///
/// Thresholds are policy, not physical capability. Available braking/thrust is
/// supplied by the ship dynamics model when the predictive supervisor is added.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct FlightSafetyProfile {
    warning_time_to_contact_seconds: f64,
    emergency_time_to_contact_seconds: f64,
    preferred_contact_speed_metres_per_second: f64,
    emergency_contact_speed_metres_per_second: f64,
    minimum_prepared_interaction_horizon_seconds: f64,
    /// Reserve braking capacity for stale observations and chart handoff.
    emergency_braking_capacity_fraction: f64,
}

impl FlightSafetyProfile {
    pub const fn spacecraft() -> Self {
        Self {
            warning_time_to_contact_seconds: 8.0,
            emergency_time_to_contact_seconds: 2.0,
            preferred_contact_speed_metres_per_second: 8.0,
            emergency_contact_speed_metres_per_second: 100.0,
            minimum_prepared_interaction_horizon_seconds: 4.0,
            emergency_braking_capacity_fraction: 0.8,
        }
    }

    pub const fn warning_time_to_contact_seconds(self) -> f64 {
        self.warning_time_to_contact_seconds
    }

    pub const fn emergency_time_to_contact_seconds(self) -> f64 {
        self.emergency_time_to_contact_seconds
    }

    pub const fn preferred_contact_speed_metres_per_second(self) -> f64 {
        self.preferred_contact_speed_metres_per_second
    }

    pub const fn emergency_contact_speed_metres_per_second(self) -> f64 {
        self.emergency_contact_speed_metres_per_second
    }

    pub const fn minimum_prepared_interaction_horizon_seconds(self) -> f64 {
        self.minimum_prepared_interaction_horizon_seconds
    }

    pub const fn emergency_braking_capacity_fraction(self) -> f64 {
        self.emergency_braking_capacity_fraction
    }
}

impl Default for FlightSafetyProfile {
    fn default() -> Self {
        Self::spacecraft()
    }
}

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlightSafetyLevel {
    #[default]
    Nominal,
    Advisory,
    Emergency,
}

/// Output of the flight safety supervisor. It observes canonical velocity and
/// body-relative geometry; it never applies a physical contact response.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct FlightSafetyState {
    level: FlightSafetyLevel,
    time_to_contact_seconds: Option<f64>,
    closing_speed_metres_per_second: f64,
    required_deceleration_metres_per_second2: f64,
    interaction_ready: bool,
}

impl FlightSafetyState {
    pub fn evaluate(
        clearance_metres: f64,
        radial_outward: DVec3,
        velocity_metres_per_second: DVec3,
        interaction_ready: bool,
        available_deceleration_metres_per_second2: f64,
        target_speed_metres_per_second: f64,
        profile: FlightSafetyProfile,
    ) -> Self {
        let outward = radial_outward.normalize_or_zero();
        let closing = if outward == DVec3::ZERO || !velocity_metres_per_second.is_finite() {
            0.0
        } else {
            (-velocity_metres_per_second.dot(outward)).max(0.0)
        };
        let clearance = clearance_metres.max(0.0);
        let time_to_contact =
            (closing > f64::EPSILON && clearance_metres.is_finite()).then(|| clearance / closing);
        let required_deceleration = if closing > 0.0 {
            ((closing * closing - target_speed_metres_per_second.max(0.0).powi(2))
                / (2.0 * clearance.max(1.0)))
            .max(0.0)
        } else {
            0.0
        };
        let available_braking = available_deceleration_metres_per_second2.max(0.0)
            * profile.emergency_braking_capacity_fraction.clamp(0.0, 1.0);
        let braking_exceeded =
            required_deceleration > 0.0 && required_deceleration >= available_braking;
        let level = match time_to_contact {
            Some(ttc)
                if braking_exceeded
                    || ttc <= profile.emergency_time_to_contact_seconds
                    || (!interaction_ready
                        && ttc <= profile.minimum_prepared_interaction_horizon_seconds) =>
            {
                FlightSafetyLevel::Emergency
            }
            Some(ttc) if ttc <= profile.warning_time_to_contact_seconds => {
                FlightSafetyLevel::Advisory
            }
            _ => FlightSafetyLevel::Nominal,
        };
        Self {
            level,
            time_to_contact_seconds: time_to_contact,
            closing_speed_metres_per_second: closing,
            required_deceleration_metres_per_second2: required_deceleration,
            interaction_ready,
        }
    }

    pub const fn level(self) -> FlightSafetyLevel {
        self.level
    }

    pub const fn time_to_contact_seconds(self) -> Option<f64> {
        self.time_to_contact_seconds
    }

    pub const fn closing_speed_metres_per_second(self) -> f64 {
        self.closing_speed_metres_per_second
    }

    pub const fn required_deceleration_metres_per_second2(self) -> f64 {
        self.required_deceleration_metres_per_second2
    }

    pub const fn interaction_ready(self) -> bool {
        self.interaction_ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::DVec3;

    #[test]
    fn approaching_unready_surface_triggers_emergency_before_contact() {
        let state = FlightSafetyState::evaluate(
            1_000.0,
            DVec3::X,
            DVec3::NEG_X * 500.0,
            false,
            35.0,
            8.0,
            FlightSafetyProfile::spacecraft(),
        );
        assert_eq!(state.level(), FlightSafetyLevel::Emergency);
        assert_eq!(state.time_to_contact_seconds(), Some(2.0));
        assert!(!state.interaction_ready());
    }

    #[test]
    fn receding_from_surface_does_not_trigger_contact_warning() {
        let state = FlightSafetyState::evaluate(
            10.0,
            DVec3::X,
            DVec3::X * 500.0,
            false,
            35.0,
            8.0,
            FlightSafetyProfile::spacecraft(),
        );
        assert_eq!(state.level(), FlightSafetyLevel::Nominal);
        assert_eq!(state.time_to_contact_seconds(), None);
    }

    #[test]
    fn extreme_speed_triggers_dropout_before_entering_local_chart() {
        let state = FlightSafetyState::evaluate(
            1.0e9,
            DVec3::X,
            DVec3::NEG_X * 1.0e9,
            false,
            35.0,
            8.0,
            FlightSafetyProfile::spacecraft(),
        );
        assert_eq!(state.level(), FlightSafetyLevel::Emergency);
        assert_eq!(state.time_to_contact_seconds(), Some(1.0));
    }
}
