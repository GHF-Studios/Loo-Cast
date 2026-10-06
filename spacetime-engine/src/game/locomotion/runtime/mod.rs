//! Controlled-subject locomotion runtime.
//!
//! State resolution chooses a kernel and collision representation. Flight
//! execution then commits canonical motion or a collision-resolved local pose.
//!
//! ## Module map
//!
//! - `flight`: Controlled flight orchestration: policy, collision, and semantic commit.
//! - `state`: Regime resolution and runtime collision/motor realization.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod flight;
mod state;

pub(super) use flight::flight_movement;
pub(super) use state::{resolve_locomotion_state, sync_locomotion_runtime};
