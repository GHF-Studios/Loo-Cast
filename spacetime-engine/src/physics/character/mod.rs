//! Kinematic character movement.
//!
//! The motor owns movement semantics while Avian provides collision queries.
//! Camera, local-player input, networking and prediction are adapters layered
//! above this module.
//!
//! ## Module map
//!
//! - `config`: Physical character dimensions and movement tuning.
//! - `controller`: Character-motor orchestration.
//! - `devtools`: Character-controller developer visualization.
//! - `frame`: Maintain character control and locomotion frames from gravity and contact.
//! - `input`: Represent the movement intent consumed by the character solver.
//! - `math`: Character movement and support-query math.
//! - `plugin`: Install character-physics systems and their schedule ordering.
//! - `state`: Persistent contact and movement state for character physics.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod config;
mod controller;
pub(crate) mod devtools;
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
