//! Kinematic character movement.
//!
//! The motor owns movement semantics while Avian provides collision queries.
//! Camera, local-player input, networking and prediction are adapters layered
//! above this module.

mod config;
mod controller;
mod input;
mod math;
mod plugin;
mod state;

pub use config::*;
pub use input::*;
pub use math::*;
pub use plugin::*;
pub use state::*;
