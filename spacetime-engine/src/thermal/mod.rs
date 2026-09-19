//! Reusable systemic thermal state, spatial refinement and combustion.
//!
//! Game-specific consequences and presentation adapt this domain without
//! becoming dependencies of the domain.

mod coupling;
pub(crate) mod devtools;
mod domain;
mod simulation;
mod spatial;
pub(crate) mod world_draw;

pub use domain::*;
pub use spatial::*;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermalSet {
    SpatialInput,
    Lumped,
    SpatialOutput,
}

pub struct ThermalCorePlugin;

impl Plugin for ThermalCorePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                ThermalSet::SpatialInput,
                ThermalSet::Lumped,
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
