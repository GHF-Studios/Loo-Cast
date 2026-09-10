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