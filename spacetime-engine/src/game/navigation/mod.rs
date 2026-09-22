//! Generic controlled-subject navigation/travel state.
//!
//! These components describe travel intent, environment-derived envelopes and
//! approach/refinement progress. They are independent from player identity and
//! from whichever Scale Slice currently realizes the subject.

use bevy::prelude::*;

use crate::spatial::SpatialScale;

/// Dimensionless commanded pace. `1.0` means the natural baseline selected by
/// the current locomotion/navigation policy.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct TravelPace {
    pub multiplier: f32,
}

impl TravelPace {
    pub const DEFAULT_MULTIPLIER: f32 = 1.0;

    pub fn character_units_per_second(self, base_speed: f32) -> f32 {
        base_speed.max(0.0) * self.multiplier.max(0.0)
    }
}

impl Default for TravelPace {
    fn default() -> Self {
        Self {
            multiplier: Self::DEFAULT_MULTIPLIER,
        }
    }
}

/// Canonical SI movement policy derived from semantic navigation context.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct TravelEnvelope {
    pub manual_speed_metres_per_second: f64,
    pub cruise_default_speed_metres_per_second: f64,
    pub cruise_max_speed_metres_per_second: f64,
    pub medium_speed_cap_metres_per_second: Option<f64>,
    pub lookahead_metres: f64,
    pub required_resolution_metres: f64,
}

impl Default for TravelEnvelope {
    fn default() -> Self {
        Self {
            manual_speed_metres_per_second: 100.0,
            cruise_default_speed_metres_per_second: 250_000_000.0,
            cruise_max_speed_metres_per_second: 10_000_000_000.0,
            medium_speed_cap_metres_per_second: None,
            lookahead_metres: 1_000.0,
            required_resolution_metres: 1_000.0,
        }
    }
}

/// Environment/navigation telemetry consumed by locomotion policy and HUD.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct TravelState {
    pub nearest_body_clearance_scale0: Option<f64>,
    pub nearest_body_radius_scale0: Option<f64>,
    pub planetary_handoff_clearance_scale0: Option<f64>,
    pub planetary_handoff_available: bool,
    pub planetary_context: bool,
    pub critical_dropout: bool,
    pub local_gravity: f32,
}

impl Default for TravelState {
    fn default() -> Self {
        Self {
            nearest_body_clearance_scale0: None,
            nearest_body_radius_scale0: None,
            planetary_handoff_clearance_scale0: None,
            planetary_handoff_available: false,
            planetary_context: false,
            critical_dropout: false,
            local_gravity: 0.0,
        }
    }
}

/// Runtime state for the Cruise motion kernel.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct AdaptiveCruise {
    pub throttle: f32,
    pub speed_scale0: f64,
    pub speed_cap_scale0: f64,
    pub default_speed_scale0: f64,
    pub nearest_hard_clearance_scale0: Option<f64>,
    pub medium_speed_cap_scale0: Option<f64>,
}

impl Default for AdaptiveCruise {
    fn default() -> Self {
        Self {
            throttle: 0.0,
            speed_scale0: 0.0,
            speed_cap_scale0: 0.0,
            default_speed_scale0: 0.0,
            nearest_hard_clearance_scale0: None,
            medium_speed_cap_scale0: None,
        }
    }
}

/// Semantic progress through approach refinement.
///
/// Presentation, realization demand and interaction handoff consume this state
/// independently; none of them owns it.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ApproachRefinementState {
    pub active: bool,
    pub continuous_exponent: f32,
    pub minimum_scale: SpatialScale,
    pub interaction_target_scale: SpatialScale,
    pub realization_target_scale: SpatialScale,
}

impl Default for ApproachRefinementState {
    fn default() -> Self {
        Self {
            active: false,
            continuous_exponent: SpatialScale::MAX.exponent() as f32,
            minimum_scale: SpatialScale::MAX,
            interaction_target_scale: SpatialScale::MAX,
            realization_target_scale: SpatialScale::MAX,
        }
    }
}

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TravelPace>()
            .register_type::<TravelEnvelope>()
            .register_type::<TravelState>()
            .register_type::<AdaptiveCruise>()
            .register_type::<ApproachRefinementState>();
    }
}
