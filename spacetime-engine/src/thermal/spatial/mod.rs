//! Spatial thermal refinement for finite solid bodies.
//!
//! [`crate::thermal::ThermalBody`] remains the semantic aggregate state used by existing game
//! systems. A [`ThermalField`] refines that state into local finite-volume cells
//! whose conserved energy is redistributed by Fourier conduction. This keeps the
//! first spatial slice compatible with existing combustion/injury code without
//! making ECS entities out of individual thermal cells.
//!
//! ## Module map
//!
//! - `field`: Compact finite-volume thermal field and Fourier conduction model.
//! - `material`: Thermodynamic material properties used by spatial thermal refinement.
//! - `simulation`: ECS adapter for localized thermal input and aggregate-state reconciliation.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod field;
mod material;
mod simulation;

pub use field::{ThermalCellSample, ThermalField};
pub use material::ThermalMaterial;
pub use simulation::ThermalPointImpulse;
pub(super) use simulation::configure;
