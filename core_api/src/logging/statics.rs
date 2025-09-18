use core_api_macros::export_static;
use crossbeam::queue::SegQueue;
use std::sync::atomic::AtomicU64;

use super::types::{LogEntry, LogId, ModulePath, PhysicalStoragePath, SpanPath};

export_static!(self, crate::logging::statics::LOG_ID_COUNTER: AtomicU64 = AtomicU64::new(1));
export_static!(self, crate::logging::statics::SPAN_EVENT_BUFFER: SegQueue<SpanPath> = SegQueue::new());
export_static!(self, crate::logging::statics::LOG_EVENT_BUFFER: SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)> = SegQueue::new());
