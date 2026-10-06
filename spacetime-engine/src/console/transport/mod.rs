//! Structured diagnostic transport with tracing and terminal adapters.
//!
//! ## Module map
//!
//! - `queue`: Thread-safe command and diagnostic queues, independent of ECS values.
//! - `record`: Structured records shared by overlay, tracing and terminal frontends.
//! - `terminal`: Native stdin and terminal record output.
//! - `tracing`: Bevy tracing adapter for the in-game overlay.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

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
