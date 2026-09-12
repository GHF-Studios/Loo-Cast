//! Kinematic character movement.
//!
//! The motor owns movement semantics while Avian provides collision queries.
//! Camera, local-player input, networking and prediction are adapters layered
//! above this module.

mod config;
mod controller;
mod frame;
mod input;
mod math;
mod plugin;
mod state;

pub(crate) use controller::{
    CharacterPush, MAX_DYNAMIC_CONTACT_DELTA_SPEED, dynamic_contact_delta_velocity,
};

pub use config::*;
pub use frame::*;
pub use input::*;
pub use math::*;
pub use plugin::*;
pub use state::*;
