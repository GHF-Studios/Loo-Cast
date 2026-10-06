//! Canonical swept-collision query boundary.
//!
//! Local collision remains the contact backend. This module only gathers
//! conservative future-motion evidence for providers and callers.
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
pub use runtime::UsfCollisionQuerySet;
pub(super) use runtime::configure;
