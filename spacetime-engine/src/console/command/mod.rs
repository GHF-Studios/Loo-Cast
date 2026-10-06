//! Command contracts, registration, parsing and built-ins.

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
