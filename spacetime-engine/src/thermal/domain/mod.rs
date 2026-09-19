//! Thermal/combustion domain state.
//!
//! Components here describe state and material response, never presentation.

mod body;
mod combustion;
mod injury;
mod events;

pub use body::{AMBIENT_TEMPERATURE_KELVIN, ThermalBody, ThermalSpatialSample};
pub use combustion::{CombustibleMaterial, Combustion, Fuel};
pub use events::ThermalImpulse;
pub use injury::ThermalInjury;

#[cfg(test)]
mod tests;
