//! Reusable GPU-offload coordination primitives.
//!
//! This module owns execution-domain mechanics that are already shared pressure:
//! monotonic reconstructible work identity and batched asynchronous completion
//! transport between GPU/render-world producers and main-world consumers.
//! Shader/pipeline semantics remain with each domain.
//!
//! Completion transport is reconstructible bookkeeping, never semantic
//! authority. Clients version their own work and reject stale completions.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

/// Monotonic reconstructible GPU-work identity.
///
/// Wrapping skips zero so domains may retain zero as an uninitialized sentinel.
#[derive(Debug, Default)]
pub struct GpuWorkSequence {
    next: u64,
}

impl GpuWorkSequence {
    #[inline]
    pub fn allocate(&mut self) -> u64 {
        self.next = self.next.wrapping_add(1).max(1);
        self.next
    }
}

/// Main-world/read side of an asynchronous GPU completion channel.
#[derive(Debug)]
pub struct GpuCompletionQueue<T> {
    shared: Arc<Mutex<VecDeque<T>>>,
}

/// GPU/render-world write side of an asynchronous completion channel.
#[derive(Debug, Clone)]
pub struct GpuCompletionSink<T> {
    shared: Arc<Mutex<VecDeque<T>>>,
}

impl<T> GpuCompletionQueue<T> {
    pub fn channel() -> (Self, GpuCompletionSink<T>) {
        let shared = Arc::new(Mutex::new(VecDeque::new()));
        (
            Self {
                shared: Arc::clone(&shared),
            },
            GpuCompletionSink { shared },
        )
    }

    /// Drain without allocating an intermediate collection.
    pub fn drain_with(&self, mut consume: impl FnMut(T)) {
        let mut queue = self.shared.lock().expect("GPU completion queue poisoned");
        for value in queue.drain(..) {
            consume(value);
        }
    }
}

impl<T> GpuCompletionSink<T> {
    /// Publish a whole GPU/render batch with one synchronization acquisition.
    pub fn extend(&self, values: impl IntoIterator<Item = T>) {
        let mut queue = self.shared.lock().expect("GPU completion queue poisoned");
        queue.extend(values);
    }
}
