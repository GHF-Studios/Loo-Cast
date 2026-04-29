//! Compile-time metadata contracts.
//!
//! Files in this module describe trait-level metadata capabilities (what a
//! metadata item must provide). They are converted into concrete runtime
//! metadata by counterparts in `meta::monomorphized`.

pub mod abstract_primitive;
pub mod function;
pub mod generic_;
pub mod impl_;
pub mod module;
pub mod trait_;
pub mod type_;
