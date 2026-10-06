//! Flight-domain policy, contact state, safety state and stable telemetry.
//!
//! Flight sits above navigation and locomotion. The model owns cockpit-facing
//! state; the runtime only copies already-resolved simulation state into it.

use crate::game::{GameSet, PresentationSet};
use bevy::prelude::*;

mod model;
mod runtime;

pub use model::*;
use runtime::sync_flight_telemetry;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlightSet {
    Telemetry,
}

pub struct FlightPlugin;

impl Plugin for FlightPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlightMode>()
            .register_type::<FlightContactState>()
            .register_type::<FlightLandingOpportunity>()
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
