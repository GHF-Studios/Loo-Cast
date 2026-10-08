//! Subject safety policy and supervisor output.

use bevy::prelude::*;

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
}

impl FlightSafetyProfile {
    pub const fn spacecraft() -> Self {
        Self {
            warning_time_to_contact_seconds: 8.0,
            emergency_time_to_contact_seconds: 2.0,
            preferred_contact_speed_metres_per_second: 8.0,
            emergency_contact_speed_metres_per_second: 100.0,
            minimum_prepared_interaction_horizon_seconds: 4.0,
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

/// Output of the flight safety supervisor.
///
/// The first structural tranche only establishes ownership. The next tranche
/// will populate this from closing velocity, TTC, braking capability and USF
/// interaction readiness.
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
