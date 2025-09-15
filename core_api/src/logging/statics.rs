use crate::statics::get_ref;
use crossbeam::queue::SegQueue;
use std::sync::atomic::AtomicU64;

use super::types::{LogEntry, LogId, ModulePath, PhysicalStoragePath, SpanPath};

/// Registry key: "log_id_counter"
pub fn log_id_counter() -> &'static AtomicU64 {
    get_ref("log_id_counter")
}

/// Registry key: "span_event_buffer"
pub fn span_event_buffer() -> &'static SegQueue<SpanPath> {
    get_ref("span_event_buffer")
}

/// Registry key: "log_event_buffer"
pub fn log_event_buffer() -> &'static SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)> {
    get_ref("log_event_buffer")
}
