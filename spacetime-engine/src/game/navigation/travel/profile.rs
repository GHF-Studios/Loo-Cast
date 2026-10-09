//! Subject capabilities and configured travel policy.

use bevy::prelude::*;

/// Navigation facilities installed on a subject.
///
/// Cruise is travel assistance over an existing movement subject, not a
/// locomotion identity or a physical actuator.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct NavigationCapabilities {
    cruise: bool,
}

impl NavigationCapabilities {
    pub const fn spacecraft() -> Self {
        Self { cruise: true }
    }

    pub const fn cruise(self) -> bool {
        self.cruise
    }
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct ManualTravelProfile {
    pub fallback_metres_per_second: f64,
    pub minimum_metres_per_second: f64,
    pub maximum_metres_per_second: f64,
    pub characteristic_traversal_seconds: f64,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct CruiseTravelProfile {
    pub default_metres_per_second: f64,
    pub maximum_metres_per_second: f64,
    pub braking_acceleration_metres_per_second2: f64,
    pub lookahead_seconds: f64,
    pub throttle_rate_per_second: f32,
    pub speed_response: f64,
    pub charge_seconds: f32,
    pub emergency_cooldown_seconds: f32,
    pub reentry_clearance_multiplier: f64,
    pub medium_minimum_resistance: f64,
    pub maximum_medium_entry_horizon_seconds: f64,
    pub default_medium_entry_horizon_seconds: f64,
    pub maximum_medium_feature_horizon_seconds: f64,
    pub default_medium_feature_horizon_seconds: f64,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct PlanetaryTravelProfile {
    pub handoff_radius_fraction: f64,
    pub handoff_minimum_metres: f64,
    pub handoff_maximum_metres: f64,
    pub capture_speed_minimum_metres_per_second: f64,
    pub capture_speed_maximum_metres_per_second: f64,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct ApproachTravelProfile {
    pub activation_radii: f64,
    pub refinement_rate_decades_per_second: f32,
    pub resolution_divisor: f64,
    pub interaction_handoff_coverage_radius_native: f32,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct FlightDynamicsProfile {
    pub local_acceleration_metres_per_second2: f32,
    /// Maximum translational braking acceleration available to ship RCS damping.
    pub rcs_braking_acceleration_metres_per_second2: f32,
    pub boost_multiplier: f32,
    /// Maximum manual local-axis pitch rate.
    pub pitch_rate_radians_per_second: f32,
    /// Maximum manual local-axis yaw rate.
    pub yaw_rate_radians_per_second: f32,
    /// Maximum manual local-axis roll rate.
    pub roll_rate_radians_per_second: f32,
    /// Slew limit used by target-orientation controllers such as autopilot.
    pub target_attitude_response_radians_per_second: f32,
}

/// Subject-owned travel/navigation policy.
///
/// This is deliberately data rather than a collection of global constants.
/// Different ships, EVA suits, creatures, drones or modded actors can expose
/// different envelopes while sharing the same navigation/runtime machinery.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct TravelProfile {
    pub manual: ManualTravelProfile,
    pub cruise: CruiseTravelProfile,
    pub planetary: PlanetaryTravelProfile,
    pub approach: ApproachTravelProfile,
    pub flight: FlightDynamicsProfile,
}

impl TravelProfile {
    pub const fn character() -> Self {
        Self {
            manual: ManualTravelProfile {
                fallback_metres_per_second: 100.0,
                minimum_metres_per_second: 8.0,
                maximum_metres_per_second: 2_500.0,
                characteristic_traversal_seconds: 90.0,
            },
            cruise: CruiseTravelProfile {
                default_metres_per_second: 250_000_000.0,
                maximum_metres_per_second: 10_000_000_000.0,
                braking_acceleration_metres_per_second2: 60_000.0,
                lookahead_seconds: 8.0,
                throttle_rate_per_second: 0.45,
                speed_response: 1.4,
                charge_seconds: 2.0,
                emergency_cooldown_seconds: 8.0,
                reentry_clearance_multiplier: 1.75,
                medium_minimum_resistance: 0.01,
                maximum_medium_entry_horizon_seconds: 4.0,
                default_medium_entry_horizon_seconds: 12.0,
                maximum_medium_feature_horizon_seconds: 1.5,
                default_medium_feature_horizon_seconds: 5.0,
            },
            planetary: PlanetaryTravelProfile {
                handoff_radius_fraction: 0.12,
                handoff_minimum_metres: 20_000.0,
                handoff_maximum_metres: 750_000.0,
                capture_speed_minimum_metres_per_second: 250.0,
                capture_speed_maximum_metres_per_second: 2_500.0,
            },
            approach: ApproachTravelProfile {
                activation_radii: 256.0,
                refinement_rate_decades_per_second: 6.0,
                resolution_divisor: 4.0,
                interaction_handoff_coverage_radius_native: 32.0,
            },
            flight: FlightDynamicsProfile {
                local_acceleration_metres_per_second2: 35.0,
                rcs_braking_acceleration_metres_per_second2: 25.0,
                boost_multiplier: 4.0,
                pitch_rate_radians_per_second: 1.4,
                yaw_rate_radians_per_second: 1.1,
                roll_rate_radians_per_second: 1.8,
                target_attitude_response_radians_per_second: 2.0,
            },
        }
    }

    pub const fn spacecraft() -> Self {
        Self::character()
    }

    pub fn planetary_handoff_clearance(self, radius_metres: f64) -> f64 {
        (radius_metres * self.planetary.handoff_radius_fraction).clamp(
            self.planetary.handoff_minimum_metres,
            self.planetary.handoff_maximum_metres,
        )
    }

    pub fn planetary_capture_speed(self, radius_metres: f64) -> f64 {
        radius_metres.sqrt().clamp(
            self.planetary.capture_speed_minimum_metres_per_second,
            self.planetary.capture_speed_maximum_metres_per_second,
        )
    }
}

impl Default for TravelProfile {
    fn default() -> Self {
        Self::character()
    }
}
