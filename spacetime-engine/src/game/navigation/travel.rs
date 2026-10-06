//! Subject-owned travel profiles, envelope, body context, and approach state.

use crate::spatial::{SpatialScale, UsfPosition};
use bevy::prelude::*;

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
    pub nearest_body_clearance_scale0: Option<f64>,
    pub nearest_body_radius_scale0: Option<f64>,
    pub planetary_handoff_clearance_scale0: Option<f64>,
    pub planetary_handoff_available: bool,
    pub critical_dropout: bool,
    pub cruise_entry_available: bool,
}

impl Default for TravelState {
    fn default() -> Self {
        Self {
            nearest_body_clearance_scale0: None,
            nearest_body_radius_scale0: None,
            planetary_handoff_clearance_scale0: None,
            planetary_handoff_available: false,
            critical_dropout: false,
            cruise_entry_available: true,
        }
    }
}

/// One resolved primary hard body for the current navigation subject.
///
/// This is travel geometry and identity only. Physical fields such as gravity
/// are queried from their own domains and must not be smuggled through
/// navigation state.
#[derive(Component, Debug, Clone, Copy)]
pub struct PrimaryBodyContext {
    entity: Option<Entity>,
    center: UsfPosition,
    radius_metres: f64,
    reference_scale: SpatialScale,
    center_distance_metres: f64,
    clearance_metres: f64,
}

impl Default for PrimaryBodyContext {
    fn default() -> Self {
        Self {
            entity: None,
            center: UsfPosition::zero(SpatialScale::MAX),
            radius_metres: 0.0,
            reference_scale: SpatialScale::MAX,
            center_distance_metres: f64::INFINITY,
            clearance_metres: f64::INFINITY,
        }
    }
}

impl PrimaryBodyContext {
    pub fn resolved(
        entity: Entity,
        center: UsfPosition,
        radius_metres: f64,
        reference_scale: SpatialScale,
        center_distance_metres: f64,
        clearance_metres: f64,
    ) -> Self {
        Self {
            entity: Some(entity),
            center,
            radius_metres,
            reference_scale,
            center_distance_metres,
            clearance_metres,
        }
    }

    pub const fn entity(self) -> Option<Entity> {
        self.entity
    }

    pub const fn center(self) -> UsfPosition {
        self.center
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }

    pub const fn reference_scale(self) -> SpatialScale {
        self.reference_scale
    }

    pub const fn center_distance_metres(self) -> f64 {
        self.center_distance_metres
    }

    pub const fn clearance_metres(self) -> f64 {
        self.clearance_metres
    }

    pub const fn is_resolved(self) -> bool {
        self.entity.is_some()
    }
}

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TravelAssistance {
    #[default]
    Manual,
    Cruise,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelAssistanceTransitionReason {
    PilotRequest,
    PilotDisengaged,
    CriticalApproach,
}

/// Pilot-selected travel assistance. The underlying locomotion regime and
/// canonical motion state continue through engagement and dropout.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct TravelAssistanceState {
    mode: TravelAssistance,
    last_transition: Option<TravelAssistanceTransitionReason>,
}

impl TravelAssistanceState {
    pub const fn mode(self) -> TravelAssistance {
        self.mode
    }
    pub const fn last_transition(self) -> Option<TravelAssistanceTransitionReason> {
        self.last_transition
    }

    pub fn engage_cruise(&mut self) {
        self.mode = TravelAssistance::Cruise;
        self.last_transition = Some(TravelAssistanceTransitionReason::PilotRequest);
    }

    pub fn disengage(&mut self, reason: TravelAssistanceTransitionReason) {
        self.mode = TravelAssistance::Manual;
        self.last_transition = Some(reason);
    }
}

/// Runtime state for explicit Cruise travel assistance.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct AdaptiveCruise {
    pub was_active: bool,
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
            was_active: false,
            throttle: 0.0,
            speed_scale0: 0.0,
            speed_cap_scale0: 0.0,
            default_speed_scale0: 0.0,
            nearest_hard_clearance_scale0: None,
            medium_speed_cap_scale0: None,
        }
    }
}

/// Semantic progress through future capability refinement.
///
/// This state owns readiness/refinement only. Interaction Scale is controlled
/// elsewhere by the controlled manifestation's explicit Scale affinity.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ApproachRefinementState {
    pub active: bool,
    pub minimum_scale: SpatialScale,
    pub realization_target_scale: SpatialScale,
}

impl Default for ApproachRefinementState {
    fn default() -> Self {
        Self {
            active: false,
            minimum_scale: SpatialScale::MAX,
            realization_target_scale: SpatialScale::MAX,
        }
    }
}
