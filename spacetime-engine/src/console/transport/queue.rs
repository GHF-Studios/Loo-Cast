//! Thread-safe command and diagnostic queues, independent of ECS values.

use super::record::ConsoleRecord;
use crate::console::ConsoleCommandSource;
use bevy::prelude::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, MutexGuard},
};

const MAX_PENDING_RECORDS: usize = 4_096;

#[derive(Debug, Clone)]
pub(in crate::console) struct ConsoleCommandSubmission {
    pub(in crate::console) source: ConsoleCommandSource,
    pub(in crate::console) raw: String,
}

/// Thread-safe transport shared by tracing, stdin, ECS dispatch and the overlay.
///
/// The queues contain no ECS values and never borrow the [`World`].
#[derive(Resource, Clone, Default)]
pub(crate) struct ConsoleTransport {
    commands: Arc<Mutex<VecDeque<ConsoleCommandSubmission>>>,
    records: Arc<Mutex<VecDeque<ConsoleRecord>>>,
}

impl ConsoleTransport {
    pub(crate) fn submit(&self, source: ConsoleCommandSource, raw: impl Into<String>) {
        let raw = raw.into();
        if raw.trim().is_empty() {
            return;
        }
        lock_recover(&self.commands).push_back(ConsoleCommandSubmission { source, raw });
    }

    pub(in crate::console) fn drain_commands(&self) -> Vec<ConsoleCommandSubmission> {
        lock_recover(&self.commands).drain(..).collect()
    }

    pub(in crate::console) fn publish(&self, record: ConsoleRecord) {
        let mut records = lock_recover(&self.records);
        while records.len() >= MAX_PENDING_RECORDS {
            records.pop_front();
        }
        records.push_back(record);
    }

    pub(in crate::console) fn drain_records(&self) -> Vec<ConsoleRecord> {
        lock_recover(&self.records).drain(..).collect()
    }
}

fn lock_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
