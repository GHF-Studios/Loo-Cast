//! Rhai binding metadata model.
//!
//! Split intentionally into:
//! - `generic/*`: compile-time metadata traits/abstractions describing intent.
//! - `monomorphized/*`: concrete runtime payloads collected into the binding graph.
//!
//! This separation lets us author metadata declaratively while still resolving to
//! deterministic runtime structures.

pub mod abstract_;
pub mod generic;
pub mod monomorphized;
pub mod registry;
