//! Structured diagnostic transport with tracing and terminal adapters.

mod queue;
mod record;
mod terminal;
mod tracing;

pub(crate) use queue::ConsoleTransport;
pub(super) use record::{
    ConsoleLogLevel, ConsoleRecord, ConsoleRecordKind, ConsoleRecordOrigin, timestamp_label,
};
#[cfg(not(target_arch = "wasm32"))]
pub(super) use terminal::start_terminal_input;
pub(super) use terminal::write_terminal_record;
pub(crate) use tracing::console_log_layer;
