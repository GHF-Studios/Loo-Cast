//! Alpha bootstrap stub for legacy `core_mod`.
//!
//! Legacy `core_mod` combined asset ownership plus runtime API integration.
//! In alpha bootstrap we keep only the crate-level identity surface.
//!
//! Tech glossary anchors:
//!
//! - `docs/glossary/Scale Contract Runtime Notes.md`
//! - `docs/glossary/USF Runtime Evolution Lifecycle Notes.md`
//! - `docs/glossary/USF Instantiation Script Profile Notes.md`
//! - `docs/glossary/Capability Role and State Authority Notes.md`
//! - `docs/glossary/Rhai Generic Dispatch Policy Notes.md`
//! - `docs/glossary/Rhai Value Semantics and AccessCell Notes.md`
//! - `docs/glossary/USF Math Raw Model Foundation Notes.md`
//! - `docs/glossary/USF Position Stack and Overflow Policy Notes.md`

pub mod spec;

pub fn crate_identity() -> &'static str {
    "core_mod"
}

// Legacy lower-level module trees are intentionally not copied yet.
// pub mod script_channels;
