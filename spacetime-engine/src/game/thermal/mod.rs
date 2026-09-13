//! Systemic thermal state and combustion.
//!
//! The first vertical slice is intentionally small but keeps the causal layers
//! separate:
//!
//! `thermal energy -> temperature -> combustion -> heat propagation`
//!
//! Domain-specific consequences then adapt that state independently. Biological
//! thermal injury emits generic combat [`Damage`](crate::game::combat::Damage),
//! while flame geometry/light is derived presentation only.

mod coupling;
pub(crate) mod observability;
mod domain;
mod presentation;
mod simulation;
mod spatial;

pub use domain::*;
pub use spatial::*;

use bevy::prelude::*;

use super::SimulationSet;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ThermalSet {
    SpatialInput,
    Lumped,
    SpatialOutput,
}

pub struct ThermalPlugin;

impl Plugin for ThermalPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                ThermalSet::SpatialInput,
                ThermalSet::Lumped,
                ThermalSet::SpatialOutput,
            )
                .chain()
                .in_set(SimulationSet::Phenomena),
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
        presentation::configure(app);
    }
}
