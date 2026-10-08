//! Flight policy, contact and safety state, and derived telemetry.
//!
//! Flight resolves typed control requests into subject-owned actuator and
//! autopilot state. Presentation consumes telemetry from the resolved runtime.
//!
//! ## Integration
//!
//! FlightControlRequest is the policy ingress for propulsion, reaction control, and autopilot. The
//! flight resolver owns actuator changes; telemetry is derived from resolved simulation state.
//!
//! ## Module map
//!
//! - `model`: Subject-owned flight policy and read-only operational snapshots.
//! - `runtime`: Resolve flight control requests and publish derived flight telemetry.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use crate::game::{GameSet, PresentationSet};
use bevy::{app::RunFixedMainLoop, prelude::*};

mod model;
mod runtime;

pub use model::*;
use runtime::{apply_attitude_autopilot, resolve_flight_control_requests, sync_flight_telemetry};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlightSet {
    Control,
    Telemetry,
}

pub struct FlightPlugin;

impl Plugin for FlightPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlightCapabilities>()
            .register_type::<FlightMode>()
            .register_type::<PilotAttitudeLaw>()
            .register_type::<AttitudeAutopilotCommand>()
            .register_type::<FlightControlCommand>()
            .add_message::<FlightControlRequest>()
            .register_type::<AttitudeAutopilotMode>()
            .register_type::<AttitudeAutopilot>()
            .register_type::<FlightContactState>()
            .register_type::<FlightLandingOpportunity>()
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
            .add_systems(
                RunFixedMainLoop,
                (resolve_flight_control_requests, apply_attitude_autopilot)
                    .chain()
                    .in_set(FlightSet::Control),
            )
            .add_systems(Update, sync_flight_telemetry.in_set(FlightSet::Telemetry));
    }
}
