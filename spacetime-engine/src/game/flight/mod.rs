//! Flight-domain policy, contact state, safety state and stable telemetry.
//!
//! Flight sits above generic locomotion and navigation:
//!
//! controller intent -> navigation/environment -> flight policy/safety
//! -> generic locomotion execution -> telemetry
//!
//! Motion kernels and Scale Slices are implementation details. Cockpit/UI/audio
//! consume [`FlightTelemetry`] instead of reaching through those internals.

use bevy::prelude::*;

use crate::{
    game::{
        GameSet, PresentationSet,
        control::LocalControlSubject,
        locomotion::{
            ControlledSubjectLocomotion, DetailedInteractionScale, LocomotionRegime,
        },
        navigation::{AdaptiveCruise, TravelState},
    },
    spatial::{SpatialScale, UsfCanonicalMotion, UsfScaleLayer},
};

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
    active: bool,
    mode: Option<FlightMode>,
    contact: FlightContactState,
    safety: FlightSafetyLevel,
    speed_metres_per_second: f64,
    throttle: f32,
    thrusters_enabled: bool,
    interaction_scale: SpatialScale,
    detailed_interaction: bool,
    clearance_metres: Option<f64>,
    local_gravity_metres_per_second2: f32,
    planetary_handoff_clearance_metres: Option<f64>,
    planetary_handoff_available: bool,
    dropout_required: bool,
    time_to_contact_seconds: Option<f64>,
    closing_speed_metres_per_second: f64,
    required_deceleration_metres_per_second2: f64,
}

impl Default for FlightTelemetry {
    fn default() -> Self {
        Self {
            active: false,
            mode: None,
            contact: FlightContactState::Airborne,
            safety: FlightSafetyLevel::Nominal,
            speed_metres_per_second: 0.0,
            throttle: 0.0,
            thrusters_enabled: false,
            interaction_scale: SpatialScale::MAX,
            detailed_interaction: false,
            clearance_metres: None,
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

    pub const fn interaction_scale(self) -> SpatialScale {
        self.interaction_scale
    }

    pub const fn detailed_interaction(self) -> bool {
        self.detailed_interaction
    }

    pub const fn clearance_metres(self) -> Option<f64> {
        self.clearance_metres
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

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlightSet {
    Telemetry,
}

pub struct FlightPlugin;

impl Plugin for FlightPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlightMode>()
            .register_type::<FlightContactState>()
            .register_type::<TraversalPolicy>()
            .register_type::<FlightSafetyProfile>()
            .register_type::<FlightSafetyLevel>()
            .register_type::<FlightSafetyState>()
            .register_type::<FlightTelemetry>()
            .configure_sets(
                Update,
                FlightSet::Telemetry
                    .in_set(GameSet::Presentation)
                    .before(PresentationSet::PrimaryView),
            )
            .add_systems(Update, sync_flight_telemetry.in_set(FlightSet::Telemetry));
    }
}

fn sync_flight_telemetry(
    mut subjects: Query<
        (
            &ControlledSubjectLocomotion,
            &DetailedInteractionScale,
            &UsfScaleLayer,
            &UsfCanonicalMotion,
            &AdaptiveCruise,
            &TravelState,
            Option<&FlightContactState>,
            Option<&FlightSafetyState>,
            &mut FlightTelemetry,
        ),
        With<LocalControlSubject>,
    >,
) {
    for (
        locomotion,
        detailed,
        layer,
        motion,
        cruise,
        travel,
        contact,
        safety,
        mut telemetry,
    ) in &mut subjects
    {
        let mode = FlightMode::from_locomotion(locomotion.regime());
        let contact = contact.copied().unwrap_or_default();
        let safety = safety.copied().unwrap_or_default();

        telemetry.active = mode.is_some() || contact.is_landed();
        telemetry.mode = mode;
        telemetry.contact = contact;
        telemetry.safety = safety.level();
        telemetry.speed_metres_per_second = motion.speed_metres_per_second();
        telemetry.throttle = if mode == Some(FlightMode::Cruise) {
            cruise.throttle
        } else {
            0.0
        };
        telemetry.thrusters_enabled = locomotion.thrusters_enabled();
        telemetry.interaction_scale = layer.scale();
        telemetry.detailed_interaction = layer.scale() == detailed.0;
        telemetry.clearance_metres = travel.nearest_body_clearance_scale0;
        telemetry.local_gravity_metres_per_second2 = travel.local_gravity;
        telemetry.planetary_handoff_clearance_metres =
            travel.planetary_handoff_clearance_scale0;
        telemetry.planetary_handoff_available = travel.planetary_handoff_available;
        telemetry.dropout_required =
            travel.critical_dropout || safety.level() == FlightSafetyLevel::Emergency;
        telemetry.time_to_contact_seconds = safety.time_to_contact_seconds();
        telemetry.closing_speed_metres_per_second =
            safety.closing_speed_metres_per_second();
        telemetry.required_deceleration_metres_per_second2 =
            safety.required_deceleration_metres_per_second2();
    }
}
