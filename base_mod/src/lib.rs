//! base_mod
//!
//! `base_mod` is the primary gameplay mod that ships with the engine. It bundles gameplay assets
//! (scripts, configs, models) and the initialization glue to register game content with the
//! runtime. Gameplay-specific scripting and bindings are provided by `base_mod_api`.
//!
//! Notes
//! - Gameplay behavior is primarily authored as assets (rhai scripts, configs) and exposed via
//!   `base_mod_api` wrappers; avoid embedding gameplay logic directly in the Rust code in `base_mod_api`.
//! - Avoid embedding gameplay asset files anywhere else but `base_mod` to keep a single
//!   authoritative source for built-in gameplay content.
//! - The crate exposes an initialization hook (via `api_initializer!`) to register mod-provided
//!   statics and assets with the runtime.

use base_mod_api::*;
use base_mod_macros::*;
use core_mod_api::*;
use core_mod_macros::api_initializer;

api_initializer!("base_mod");
