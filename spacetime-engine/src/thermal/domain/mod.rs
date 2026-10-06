//! Thermal/combustion domain state.
//!
//! Components here describe state and material response, never presentation.
//!
//! ## Module map
//!
//! - `body`: Aggregate thermal state and manifestation-space sampling marker.
//! - `combustion`: Combustible material behavior, consumable fuel and derived combustion state.
//! - `events`: Thermal domain messages.
//! - `injury`: Biological-style response policy for dangerous body temperature.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod body;
mod combustion;
mod events;
mod injury;

pub use body::{AMBIENT_TEMPERATURE_KELVIN, ThermalBody, ThermalSpatialSample};
pub use combustion::{CombustibleMaterial, Combustion, Fuel};
pub use events::ThermalImpulse;
pub use injury::ThermalInjury;
