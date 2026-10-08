//! Commanded pace, resolved envelope, and observed travel state.

use super::TravelProfile;
use bevy::prelude::*;

/// Dimensionless commanded pace. `1.0` means the natural baseline selected by
/// the current locomotion/navigation policy.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct TravelPace {
    pub multiplier: f32,
}

impl TravelPace {
    pub const DEFAULT_MULTIPLIER: f32 = 1.0;

    //
    // Pace is dimensionless controller intent. Base-2 gives exact, predictable
    // octave steps while spanning many orders of magnitude without changing
    // Scale Slice, canonical SI authority or locomotion regime.
    pub const MIN_LOG2_MULTIPLIER: f32 = -16.0;
    pub const MAX_LOG2_MULTIPLIER: f32 = 24.0;

    pub fn log2_multiplier(self) -> f32 {
        self.multiplier
            .max(2.0_f32.powf(Self::MIN_LOG2_MULTIPLIER))
            .log2()
            .clamp(Self::MIN_LOG2_MULTIPLIER, Self::MAX_LOG2_MULTIPLIER)
    }

    pub fn add_log2_steps(&mut self, steps: f32) {
        if !steps.is_finite() || steps == 0.0 {
            return;
        }
        let exponent = (self.log2_multiplier() + steps)
            .clamp(Self::MIN_LOG2_MULTIPLIER, Self::MAX_LOG2_MULTIPLIER);
        self.multiplier = 2.0_f32.powf(exponent);
    }

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
        let profile = TravelProfile::character();
        Self {
            manual_speed_metres_per_second: profile.manual.fallback_metres_per_second,
            cruise_default_speed_metres_per_second: profile.cruise.default_metres_per_second,
            cruise_max_speed_metres_per_second: profile.cruise.maximum_metres_per_second,
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
    pub nearest_body_clearance_metres: Option<f64>,
    pub nearest_body_radius_metres: Option<f64>,
    pub planetary_handoff_clearance_metres: Option<f64>,
    pub planetary_handoff_available: bool,
    pub critical_dropout: bool,
    pub cruise_entry_available: bool,
}

impl Default for TravelState {
    fn default() -> Self {
        Self {
            nearest_body_clearance_metres: None,
            nearest_body_radius_metres: None,
            planetary_handoff_clearance_metres: None,
            planetary_handoff_available: false,
            critical_dropout: false,
            cruise_entry_available: true,
        }
    }
}
