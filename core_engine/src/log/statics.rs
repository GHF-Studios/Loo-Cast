use crossbeam::queue::SegQueue;
use once_cell::sync::Lazy;
use std::sync::{atomic::AtomicU64, OnceLock};

use crate::log::types::{LogEntry, LogId, ModulePath, PhysicalStoragePath, SpanPath};

pub(super) static LOG_ID_COUNTER: OnceLock<AtomicU64> = OnceLock::new();
pub(super) static SPAN_EVENT_BUFFER: Lazy<SegQueue<SpanPath>> = Lazy::new(SegQueue::new);
pub(super) static LOG_EVENT_BUFFER: Lazy<SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)>> = Lazy::new(SegQueue::new);
