//! Command contracts, registration, parsing and built-ins.
//!
//! ## Module map
//!
//! - `builtin`: Built-in commands over the shared registry.
//! - `contract`: Typed command ingress and result contract shared by frontends.
//! - `parse`: Console command-line parsing and canonical path normalization.
//! - `registry`: Registered command paths, aliases and argument-completion ownership.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod builtin;
mod contract;
mod parse;
mod registry;

pub(super) use builtin::{clear_command, echo_command, help_command};
pub use contract::{
    ConsoleArgumentCompletion, ConsoleCommandHandler, ConsoleCommandInvocation,
    ConsoleCommandResult, ConsoleCommandSource, ConsoleCommandSpec, ConsoleFocusDisposition,
};
pub(super) use parse::parse_command;
pub use registry::{AppConsoleExt, ConsoleCommandRegistry};
