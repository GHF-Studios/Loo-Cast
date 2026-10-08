//! Input completion over registered command paths and runtime variables.
//!
//! Scanning locates the cursor site, candidate lookup consults registries,
//! and editing alone mutates the caller's input buffer.

mod candidate;
mod edit;
mod scan;

pub(super) use candidate::completion_candidates;
pub(super) use edit::{char_to_byte_index, complete_command_input};
