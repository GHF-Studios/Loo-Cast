//! Alpha bootstrap stub for legacy `core_mod`.
//!
//! Legacy `core_mod` combined asset ownership plus runtime API integration.
//! In alpha bootstrap we keep only the crate-level identity surface.

pub fn crate_identity() -> &'static str {
    "core_mod"
}

// Legacy lower-level module trees are intentionally not copied yet.
// pub mod script_channels;
