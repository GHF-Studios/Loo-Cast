use crossbeam::queue::SegQueue;
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicU64;

use crate::statics::get_ref;

use super::types::{LogEntry, LogId, ModulePath, PhysicalStoragePath, SpanPath};

pub fn init_log_id_counter() -> AtomicU64 {
    AtomicU64::new(1)
}

pub fn init_span_event_buffer() -> SegQueue<SpanPath> {
    SegQueue::new()
}

pub fn init_log_event_buffer() -> SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)> {
    SegQueue::new()
}

pub fn log_id_counter() -> &'static AtomicU64 {
    get_ref("log_id_counter")
}

pub fn span_event_buffer() -> &'static SegQueue<SpanPath> {
    get_ref("span_event_buffer")
}

pub fn log_event_buffer() -> &'static SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)> {
    get_ref("log_event_buffer")
}
