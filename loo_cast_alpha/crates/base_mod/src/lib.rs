//! Alpha bootstrap stub for legacy `base_mod`.
//!
//! Legacy `base_mod` held gameplay-facing assets and integration bindings.
//! In alpha bootstrap we keep only a crate identity surface.

pub fn crate_identity() -> &'static str {
    "base_mod"
}

// Legacy lower-level gameplay modules are intentionally not copied yet.
