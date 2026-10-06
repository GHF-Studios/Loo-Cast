//! Detached local debug camera and observer-policy adapter.
//!
//! #57 detached-debug-freecam-v2
//!
//! Freecam never becomes gameplay/interaction authority. It may optionally
//! become the *presentation observer* through `UsfViewObservationOverride`,
//! while the gameplay subject continues to own canonical simulation position,
//! collision, refinement and dense spatial/materialization demand.

mod movement;
mod observer;
mod policy;

pub(in crate::game::player) use movement::update_freecam;
pub(in crate::game::player) use observer::sync_freecam_observer_policy;
pub(crate) use policy::{DebugFreecam, FreecamControlPolicy, FreecamProjectionPolicy};
