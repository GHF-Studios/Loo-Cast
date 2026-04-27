//! Runtime metadata payloads.
//!
//! Files in this module hold concrete metadata structs pushed into the runtime
//! binding graph. They are the normalized, monomorphized form of metadata
//! contracts declared in `meta::generic`.

pub mod function;
pub mod generic_;
pub mod impl_;
pub mod module;
pub mod trait_;
pub mod type_;
