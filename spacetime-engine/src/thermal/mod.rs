//! Reusable systemic thermal state, spatial refinement and combustion.
//!
//! Game-specific consequences and presentation adapt this domain without
//! becoming dependencies of the domain.
//!
//! ## Module map
//!
//! - `coupling`: Pure thermal coupling functions shared by simulation and observability.
//! - `devtools`: Structured semantic inspection and contextual gizmo support for thermal state.
//! - `domain`: Thermal/combustion domain state.
//! - `presentation`: Derived presentation of combustion.
//! - `simulation`: Thermal energy evolution and combustion propagation.
//! - `spatial`: Spatial thermal refinement for finite solid bodies.
//! - `world_draw`: Thermal developer visualizations.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod coupling;
pub(crate) mod devtools;
mod domain;
mod presentation;
mod simulation;
mod spatial;
pub(crate) mod world_draw;

pub use domain::*;
pub use spatial::*;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermalPresentationSet {
    Derived,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermalSet {
    SpatialInput,
    Evolution,
    SpatialOutput,
}

pub struct ThermalCorePlugin;

impl Plugin for ThermalCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                ThermalSet::SpatialInput,
                ThermalSet::Evolution,
                ThermalSet::SpatialOutput,
            )
                .chain(),
        )
        .add_message::<ThermalImpulse>()
        .register_type::<ThermalBody>()
        .register_type::<ThermalSpatialSample>()
        .register_type::<CombustibleMaterial>()
        .register_type::<Fuel>()
        .register_type::<Combustion>()
        .register_type::<ThermalInjury>();

        spatial::configure(app);
        simulation::configure(app);
    }
}

pub struct ThermalPresentationPlugin;

impl Plugin for ThermalPresentationPlugin {
    fn build(&self, app: &mut App) {
        presentation::configure(app);
    }
}
