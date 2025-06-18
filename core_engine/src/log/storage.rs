use std::sync::Arc;
use dashmap::DashMap;
use std::time::SystemTime;

use crate::log::arena::{Level};

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub ts: u64,
    pub lvl: Level,
    pub msg: Arc<str>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogId(pub u64);

#[derive(Default)]
pub struct LogStorage {
    counter: std::sync::atomic::AtomicU64,
    logs: DashMap<LogId, LogEntry>,
}

impl LogStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_log(&self, lvl: Level, msg: Arc<str>) -> LogId {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let id = LogId(self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let entry = LogEntry { ts, lvl, msg };
        self.logs.insert(id, entry);
        id
    }

    pub fn get(&self, id: &LogId) -> Option<LogEntry> {
        self.logs.get(id).map(|e| e.clone())
    }

    pub fn all_ids(&self) -> Vec<LogId> {
        self.logs.iter().map(|e| *e.key()).collect()
    }
}
