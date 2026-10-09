//! Read-only flight snapshot published for presentation consumers.

use super::{FlightContactState, FlightMode, FlightSafetyLevel};
use crate::{game::navigation::TravelAssistance, spatial::SpatialScale};
use bevy::prelude::*;

/// Stable read-only flight snapshot for HUD/cockpit/audio/VFX/debug consumers.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct FlightTelemetry {
    pub(in crate::game::flight) active: bool,
    pub(in crate::game::flight) mode: Option<FlightMode>,
    pub(in crate::game::flight) assistance: TravelAssistance,
    pub(in crate::game::flight) contact: FlightContactState,
    pub(in crate::game::flight) landing_available: bool,
    pub(in crate::game::flight) safety: FlightSafetyLevel,
    pub(in crate::game::flight) speed_metres_per_second: f64,
    pub(in crate::game::flight) forward_speed_metres_per_second: f64,
    pub(in crate::game::flight) lateral_speed_metres_per_second: f64,
    pub(in crate::game::flight) throttle: f32,
    pub(in crate::game::flight) lattice_cooldown_seconds: f32,
    pub(in crate::game::flight) lattice_charge_seconds: f32,
    pub(in crate::game::flight) thrusters_enabled: bool,
    pub(in crate::game::flight) rcs_enabled: bool,
    pub(in crate::game::flight) angular_assist_enabled: bool,
    pub(in crate::game::flight) interaction_scale: SpatialScale,
    pub(in crate::game::flight) detailed_interaction: bool,
    pub(in crate::game::flight) primary_body: Option<Entity>,
    pub(in crate::game::flight) surface_clearance_metres: Option<f64>,
    pub(in crate::game::flight) surface_collision_ready: bool,
    pub(in crate::game::flight) local_gravity_metres_per_second2: f32,
    pub(in crate::game::flight) planetary_handoff_clearance_metres: Option<f64>,
    pub(in crate::game::flight) planetary_handoff_available: bool,
    pub(in crate::game::flight) dropout_required: bool,
    pub(in crate::game::flight) time_to_contact_seconds: Option<f64>,
    pub(in crate::game::flight) closing_speed_metres_per_second: f64,
    pub(in crate::game::flight) required_deceleration_metres_per_second2: f64,
}

impl Default for FlightTelemetry {
    fn default() -> Self {
        Self {
            active: false,
            mode: None,
            assistance: TravelAssistance::Manual,
            contact: FlightContactState::Airborne,
            landing_available: false,
            safety: FlightSafetyLevel::Nominal,
            speed_metres_per_second: 0.0,
            forward_speed_metres_per_second: 0.0,
            lateral_speed_metres_per_second: 0.0,
            throttle: 0.0,
            lattice_cooldown_seconds: 0.0,
            lattice_charge_seconds: 0.0,
            thrusters_enabled: false,
            rcs_enabled: false,
            angular_assist_enabled: false,
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

    pub const fn assistance(self) -> TravelAssistance {
        self.assistance
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

    pub const fn forward_speed_metres_per_second(self) -> f64 {
        self.forward_speed_metres_per_second
    }

    pub const fn lateral_speed_metres_per_second(self) -> f64 {
        self.lateral_speed_metres_per_second
    }

    pub const fn throttle(self) -> f32 {
        self.throttle
    }

    pub const fn lattice_cooldown_seconds(self) -> f32 {
        self.lattice_cooldown_seconds
    }

    pub const fn lattice_charge_seconds(self) -> f32 {
        self.lattice_charge_seconds
    }

    pub const fn thrusters_enabled(self) -> bool {
        self.thrusters_enabled
    }

    pub const fn rcs_enabled(self) -> bool {
        self.rcs_enabled
    }

    pub const fn angular_assist_enabled(self) -> bool {
        self.angular_assist_enabled
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
        } else if matches!(self.assistance, TravelAssistance::Cruise) {
            "LATTICE CRUISE"
        } else {
            match self.mode {
                Some(mode) => mode.label(),
                None => "ON FOOT",
            }
        }
    }
}
