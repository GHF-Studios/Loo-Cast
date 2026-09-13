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

pub use domain::*;

use bevy::prelude::*;

pub struct ThermalPlugin;

impl Plugin for ThermalPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ThermalImpulse>()
            .register_type::<ThermalBody>()
            .register_type::<ThermalSpatialSample>()
            .register_type::<CombustibleMaterial>()
            .register_type::<Fuel>()
            .register_type::<Combustion>()
            .register_type::<ThermalInjury>();

        simulation::configure(app);
        presentation::configure(app);
    }
}
