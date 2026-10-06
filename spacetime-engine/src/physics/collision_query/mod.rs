//! Canonical swept-collision query boundary.
//!
//! Local collision remains the contact backend. This module only gathers
//! conservative future-motion evidence for providers and callers.

mod contract;
mod frame;
mod runtime;

pub use contract::*;
pub use frame::*;
pub use runtime::UsfCollisionQuerySet;
pub(super) use runtime::configure;
