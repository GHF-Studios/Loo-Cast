//! Subject-owned flight policy and read-only operational snapshots.

use crate::{game::locomotion::LocomotionRegime, spatial::SpatialScale};
use bevy::prelude::*;

/// Player-/pilot-facing operational flight mode.
///
/// This is deliberately NOT a motion-kernel enum. One mode may be realized by
/// different numerical kernels as interaction precision and environment change.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightMode {
    Local,
    Planetary,
    Cruise,
}

impl FlightMode {
    pub const fn from_locomotion(regime: LocomotionRegime) -> Option<Self> {
        match regime {
            LocomotionRegime::OnFoot => None,
            LocomotionRegime::LocalFlight => Some(Self::Local),
            LocomotionRegime::PlanetaryFlight => Some(Self::Planetary),
            LocomotionRegime::Cruise => Some(Self::Cruise),
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "LOCAL FLIGHT",
            Self::Planetary => "PLANETARY FLIGHT",
            Self::Cruise => "CRUISE",
        }
    }
}

/// Physical contact state is orthogonal to flight mode.
///
/// A landed ship remains a valid locomotion subject; contact merely contributes
/// a motion inhibition until launch/takeoff releases it.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum FlightContactState {
    #[default]
    Airborne,
    Landed,
}

impl FlightContactState {
    pub const fn is_landed(self) -> bool {
        matches!(self, Self::Landed)
    }

    pub fn land(&mut self) {
        *self = Self::Landed;
    }

    pub fn launch(&mut self) {
        *self = Self::Airborne;
    }
}

/// Continuously refreshed support eligibility for an explicit landing request.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlightLandingOpportunity {
    available: bool,
}

impl FlightLandingOpportunity {
    pub const fn available(self) -> bool {
        self.available
    }
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
}

/// Traversal policy is explicit rather than inferred from ship capabilities.
///
/// `Creative` is a future canonical-USF traversal path, not an overpowered
/// physical spacecraft mode.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum TraversalPolicy {
    #[default]
    Physical,
    Creative,
}

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

/// Stable read-only flight snapshot for HUD/cockpit/audio/VFX/debug consumers.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct FlightTelemetry {
    pub(super) active: bool,
    pub(super) mode: Option<FlightMode>,
    pub(super) contact: FlightContactState,
    pub(super) landing_available: bool,
    pub(super) safety: FlightSafetyLevel,
    pub(super) speed_metres_per_second: f64,
    pub(super) throttle: f32,
    pub(super) thrusters_enabled: bool,
    pub(super) rcs_enabled: bool,
    pub(super) interaction_scale: SpatialScale,
    pub(super) detailed_interaction: bool,
    pub(super) primary_body: Option<Entity>,
    pub(super) surface_clearance_metres: Option<f64>,
    pub(super) surface_collision_ready: bool,
    pub(super) local_gravity_metres_per_second2: f32,
    pub(super) planetary_handoff_clearance_metres: Option<f64>,
    pub(super) planetary_handoff_available: bool,
    pub(super) dropout_required: bool,
    pub(super) time_to_contact_seconds: Option<f64>,
    pub(super) closing_speed_metres_per_second: f64,
    pub(super) required_deceleration_metres_per_second2: f64,
}

impl Default for FlightTelemetry {
    fn default() -> Self {
        Self {
            active: false,
            mode: None,
            contact: FlightContactState::Airborne,
            landing_available: false,
            safety: FlightSafetyLevel::Nominal,
            speed_metres_per_second: 0.0,
            throttle: 0.0,
            thrusters_enabled: false,
            rcs_enabled: false,
            interaction_scale: SpatialScale::MAX,
            detailed_interaction: false,
            primary_body: None,
            surface_clearance_metres: None,
            surface_collision_ready: false,
            local_gravity_metres_per_second2: 0.0,
            planetary_handoff_clearance_metres: None,
            planetary_handoff_available: false,
            dropout_required: false,
            time_to_contact_seconds: None,
            closing_speed_metres_per_second: 0.0,
            required_deceleration_metres_per_second2: 0.0,
        }
    }
}

impl FlightTelemetry {
    pub const fn active(self) -> bool {
        self.active
    }

    pub const fn mode(self) -> Option<FlightMode> {
        self.mode
    }

    pub const fn contact(self) -> FlightContactState {
        self.contact
    }

    pub const fn landing_available(self) -> bool {
        self.landing_available
    }

    pub const fn safety(self) -> FlightSafetyLevel {
        self.safety
    }

    pub const fn speed_metres_per_second(self) -> f64 {
        self.speed_metres_per_second
    }

    pub const fn throttle(self) -> f32 {
        self.throttle
    }

    pub const fn thrusters_enabled(self) -> bool {
        self.thrusters_enabled
    }

    pub const fn rcs_enabled(self) -> bool {
        self.rcs_enabled
    }

    pub const fn interaction_scale(self) -> SpatialScale {
        self.interaction_scale
    }

    pub const fn detailed_interaction(self) -> bool {
        self.detailed_interaction
    }

    pub const fn primary_body(self) -> Option<Entity> {
        self.primary_body
    }

    pub const fn surface_clearance_metres(self) -> Option<f64> {
        self.surface_clearance_metres
    }

    pub const fn surface_collision_ready(self) -> bool {
        self.surface_collision_ready
    }

    pub const fn local_gravity_metres_per_second2(self) -> f32 {
        self.local_gravity_metres_per_second2
    }

    pub const fn planetary_handoff_clearance_metres(self) -> Option<f64> {
        self.planetary_handoff_clearance_metres
    }

    pub const fn planetary_handoff_available(self) -> bool {
        self.planetary_handoff_available
    }

    pub const fn dropout_required(self) -> bool {
        self.dropout_required
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

    pub const fn display_mode_label(self) -> &'static str {
        if self.contact.is_landed() {
            "LANDED"
        } else {
            match self.mode {
                Some(mode) => mode.label(),
                None => "ON FOOT",
            }
        }
    }
}
