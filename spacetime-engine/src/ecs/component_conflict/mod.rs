//! Register and report incompatible ECS component combinations.
//!
//! ## Module map
//!
//! - `plugin`: Install runtime component-conflict diagnostics.
//! - `registration`: Declare component-conflict rules for runtime validation.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod plugin;
mod registration;

pub use plugin::ComponentConflictPlugin;

/// Implementation surface used by generated procedural-macro output.
///
/// This module is public only because proc-macro expansions execute in the
/// consuming crate. It is not part of the supported user-facing API.
#[doc(hidden)]
pub mod __macro_support {
    pub use inventory;

    pub use super::registration::ConflictRegistration;
}
