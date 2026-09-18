//! Spatial thermal refinement for finite solid bodies.
//!
//! [`ThermalBody`] remains the semantic aggregate state used by existing game
//! systems. A [`ThermalField`] refines that state into local finite-volume cells
//! whose conserved energy is redistributed by Fourier conduction. This keeps the
//! first spatial slice compatible with existing combustion/injury code without
//! making ECS entities out of individual thermal cells.

mod field;
mod material;
mod simulation;

pub use field::{ThermalCellSample, ThermalField};
pub use material::ThermalMaterial;
pub use simulation::ThermalPointImpulse;
pub(super) use simulation::configure;

#[cfg(test)]
mod tests;
