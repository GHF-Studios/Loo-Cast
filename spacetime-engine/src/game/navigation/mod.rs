//! Generic controlled-subject navigation and travel policy.
//!
//! Navigation state belongs to the controlled subject, not to player identity
//! and not to the current Scale Slice. Profiles are canonical SI policy;
//! runtime kernels perform chart conversion only at their numerical boundary.

use bevy::{app::RunFixedMainLoop, prelude::*};

use crate::spatial::{SpatialScale, UsfPosition};

mod policy;
mod runtime;

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
    pub release_multiplier: f64,
    pub capture_speed_minimum_metres_per_second: f64,
    pub capture_speed_maximum_metres_per_second: f64,
    pub local_capture_radius_fraction: f64,
    pub local_capture_minimum_metres: f64,
    pub local_capture_maximum_metres: f64,
    pub local_release_multiplier: f64,
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
    pub orbital_acceleration_metres_per_second2: f32,
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
                release_multiplier: 1.75,
                capture_speed_minimum_metres_per_second: 250.0,
                capture_speed_maximum_metres_per_second: 2_500.0,
                local_capture_radius_fraction: 0.02,
                local_capture_minimum_metres: 10_000.0,
                local_capture_maximum_metres: 75_000.0,
                local_release_multiplier: 2.0,
            },
            approach: ApproachTravelProfile {
                activation_radii: 256.0,
                refinement_rate_decades_per_second: 6.0,
                resolution_divisor: 4.0,
                interaction_handoff_coverage_radius_native: 32.0,
            },
            flight: FlightDynamicsProfile {
                local_acceleration_metres_per_second2: 35.0,
                orbital_acceleration_metres_per_second2: 20.0,
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

    pub fn planetary_release_clearance(self, radius_metres: f64) -> f64 {
        self.planetary_handoff_clearance(radius_metres)
            * self.planetary.release_multiplier
    }

    pub fn local_flight_capture_clearance(self, radius_metres: f64) -> f64 {
        (radius_metres * self.planetary.local_capture_radius_fraction).clamp(
            self.planetary.local_capture_minimum_metres,
            self.planetary.local_capture_maximum_metres,
        )
    }

    pub fn local_flight_release_clearance(self, radius_metres: f64) -> f64 {
        self.local_flight_capture_clearance(radius_metres)
            * self.planetary.local_release_multiplier
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

/// One resolved primary hard body for the current navigation subject.
///
/// Gravity, orbital telemetry and locomotion-domain selection consume this same
/// context instead of independently rescanning celestial sources.
#[derive(Component, Debug, Clone, Copy)]
pub struct PrimaryBodyContext {
    entity: Option<Entity>,
    center: UsfPosition,
    radius_metres: f64,
    field_scale: SpatialScale,
    surface_gravity_metres_per_second2: f32,
    center_distance_metres: f64,
    clearance_metres: f64,
}

impl Default for PrimaryBodyContext {
    fn default() -> Self {
        Self {
            entity: None,
            center: UsfPosition::zero(SpatialScale::MAX),
            radius_metres: 0.0,
            field_scale: SpatialScale::MAX,
            surface_gravity_metres_per_second2: 0.0,
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
        field_scale: SpatialScale,
        surface_gravity_metres_per_second2: f32,
        center_distance_metres: f64,
        clearance_metres: f64,
    ) -> Self {
        Self {
            entity: Some(entity),
            center,
            radius_metres,
            field_scale,
            surface_gravity_metres_per_second2,
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

    pub const fn field_scale(self) -> SpatialScale {
        self.field_scale
    }

    pub const fn surface_gravity_metres_per_second2(self) -> f32 {
        self.surface_gravity_metres_per_second2
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


/// View-owned automatic presentation policy for semantic navigation.
///
/// The minimum is a content/realizer capability boundary for this view profile,
/// not a privileged USF floor. The current game defaults to S0 because the
/// present macro terrain and human-scale content are authored through metres.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct NavigationPresentationProfile {
    pub minimum_scale: SpatialScale,
    pub maximum_scale: SpatialScale,
    pub response_decades_per_second: f32,
    pub maximum_manual_bias_decades: f32,
}

impl Default for NavigationPresentationProfile {
    fn default() -> Self {
        Self {
            minimum_scale: SpatialScale::ZERO,
            maximum_scale: SpatialScale::MAX,
            response_decades_per_second: 10.0,
            maximum_manual_bias_decades: 8.0,
        }
    }
}

/// Persistent state of the automatic presentation planner.
///
/// `manual_bias_decades` is an offset on semantic automatic scale, so manual
/// zoom and automatic navigation compose instead of racing over view state.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct NavigationPresentationState {
    initialized: bool,
    automatic_target_exponent: f32,
    effective_target_exponent: f32,
    manual_bias_decades: f32,
}

impl Default for NavigationPresentationState {
    fn default() -> Self {
        Self {
            initialized: false,
            automatic_target_exponent: SpatialScale::MAX.exponent() as f32,
            effective_target_exponent: SpatialScale::MAX.exponent() as f32,
            manual_bias_decades: 0.0,
        }
    }
}

impl NavigationPresentationState {
    pub const fn initialized(self) -> bool { self.initialized }
    pub const fn automatic_target_exponent(self) -> f32 { self.automatic_target_exponent }
    pub const fn effective_target_exponent(self) -> f32 { self.effective_target_exponent }
    pub const fn manual_bias_decades(self) -> f32 { self.manual_bias_decades }

    pub fn add_manual_bias(&mut self, delta: f32) {
        if delta.is_finite() {
            self.manual_bias_decades += delta;
        }
    }
}

/// Compact end-to-end navigation/presentation telemetry.
#[derive(Resource, Debug, Clone, Copy)]
pub struct NavigationAudit {
    pub healthy: bool,
    pub subject: Option<Entity>,
    pub subject_scale: Option<SpatialScale>,
    pub primary_body: Option<Entity>,
    pub primary_clearance_metres: Option<f64>,
    pub navigation_source_scale: Option<SpatialScale>,
    pub characteristic_length_metres: f64,
    pub approach_active: bool,
    pub interaction_target_scale: Option<SpatialScale>,
    pub realization_target_scale: Option<SpatialScale>,
    pub view_exponent: f32,
    pub presentation_target_exponent: f32,
}

impl Default for NavigationAudit {
    fn default() -> Self {
        Self {
            healthy: false,
            subject: None,
            subject_scale: None,
            primary_body: None,
            primary_clearance_metres: None,
            navigation_source_scale: None,
            characteristic_length_metres: 0.0,
            approach_active: false,
            interaction_target_scale: None,
            realization_target_scale: None,
            view_exponent: SpatialScale::MAX.exponent() as f32,
            presentation_target_exponent: SpatialScale::MAX.exponent() as f32,
        }
    }
}

/// Stable semantic navigation runtime extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavigationSet {
    Observe,
    Plan,
    Publish,
}

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavigationAudit>()
            .register_type::<ManualTravelProfile>()
            .register_type::<CruiseTravelProfile>()
            .register_type::<PlanetaryTravelProfile>()
            .register_type::<ApproachTravelProfile>()
            .register_type::<FlightDynamicsProfile>()
            .register_type::<TravelProfile>()
            .register_type::<TravelPace>()
            .register_type::<TravelEnvelope>()
            .register_type::<TravelState>()
            .register_type::<AdaptiveCruise>()
            .register_type::<ApproachRefinementState>()
            .register_type::<NavigationPresentationProfile>()
            .register_type::<NavigationPresentationState>()
            .add_systems(
                RunFixedMainLoop,
                (
                    runtime::sync_navigation_context,
                    runtime::sync_travel_state,
                    runtime::sync_planetary_gravity,
                    policy::sync_travel_envelope,
                )
                    .chain()
                    .in_set(NavigationSet::Observe),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    runtime::plan_approach_refinement,
                    runtime::sync_navigation_presentation,
                )
                    .chain()
                    .in_set(NavigationSet::Plan),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    runtime::sync_approach_interaction_requirement,
                    runtime::audit_navigation_contract,
                )
                    .chain()
                    .in_set(NavigationSet::Publish),
            );
    }
}
