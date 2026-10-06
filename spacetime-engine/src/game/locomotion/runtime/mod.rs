//! Controlled-subject locomotion runtime.
//!
//! State resolution chooses a kernel and collision representation. Flight
//! execution then commits canonical motion or a collision-resolved local pose.

mod flight;
mod state;

pub(super) use flight::flight_movement;
pub(super) use state::{resolve_locomotion_state, sync_locomotion_runtime};
