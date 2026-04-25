//! core_mod
//!
//! The `core_mod` crate packages the canonical runtime data and built-in content used by the
//! engine. It contains configuration files, canonical assets (scripts, models, shaders), and a
//! small initialization layer that connects those assets to the `core_mod_api` code surface.
//!
//! Purpose
//! - Host authoritative data and asset bundles (configs, scripts-as-assets where used, art,
//!   shaders, and default models).
//! - Provide a single canonical source for built-in content and an initialization hook via
//!   `api_initializer!` that registers global statics exposed by `core_mod_api`.
//!
//! Notes
//! - Asset ownership: assets belong to `core_mod` and should be referenced by code through
//!   `core_mod_api` typed helpers when possible.
//! - Avoid embedding non-gameplay asset files anywhere else but `core_mod` to keep a single
//!   authoritative source for built-in non-gameplay "content".
//! - Keep long-form design notes in `documents/` and short usage notes near asset locations.
//!
//! TODO: add a crate-level README listing canonical config paths and asset conventions

pub use core_engine_macros;
pub use core_mod_api;

pub mod script_channels;

use core_engine_macros::api_initializer;

api_initializer!(
    "core_mod_api",
    crate::core_mod_api::config::statics::CONFIG,
    crate::core_mod_api::core::statics::TOKIO_RUNTIME,
    crate::core_mod_api::core::statics::START_TIME,
    crate::core_mod_api::logging::statics::LOG_ID_COUNTER,
    crate::core_mod_api::logging::statics::SPAN_EVENT_BUFFER,
    crate::core_mod_api::logging::statics::LOG_EVENT_BUFFER,
    crate::core_mod_api::time::statics::ELAPSED_VIRTUAL_NANOS,
    crate::core_mod_api::time::statics::PENDING_VIRTUAL_SLEEPS,
    crate::core_mod_api::workflow::statics::WORKFLOW_TOKIO_RUNTIME,
    crate::core_mod_api::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME,
);
