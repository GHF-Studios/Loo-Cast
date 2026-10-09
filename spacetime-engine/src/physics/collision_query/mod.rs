//! Canonical swept-collision query boundary.
//!
//! Local collision remains the detailed contact backend. Canonical proposals
//! are resolved before motion commits when local collider coverage is absent.
//!
//! ## Module map
//!
//! - `contract`: Canonical swept-collision contracts and one-frame query journal.
//! - `frame`: Per-frame query requests and provider observations.
//! - `runtime`: ECS collection and provider ordering for canonical collision queries.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod contract;
mod frame;
mod runtime;

pub use contract::*;
pub use frame::*;
pub(super) use runtime::configure;
pub use runtime::{UsfCollisionQuerySet, UsfProposedSweeps};
