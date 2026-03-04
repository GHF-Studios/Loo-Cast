//! Canonical runtime-facing namespace for bridge internals.
//!
//! During migration, this re-exports the legacy `script::*` runtime scaffolding.
//! New bridge code should prefer `rhai_binding::runtime::*` over `script::*`.

pub use crate::script::ecs;
pub use crate::script::rust;
pub use crate::script::usf;
