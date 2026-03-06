use crate::bevy::prelude::*;

use crate::chunk::resources::{ChunkBatchTracker, ChunkLoadGate};

pub fn run_if_chunk_load_gate_open(chunk_load_gate: Option<Res<ChunkLoadGate>>) -> bool {
    let Some(chunk_load_gate) = chunk_load_gate else {
        return true;
    };

    !chunk_load_gate.is_locked()
}

pub fn run_if_chunk_batch_running(chunk_batch_tracker: Option<Res<ChunkBatchTracker>>) -> bool {
    let Some(chunk_batch_tracker) = chunk_batch_tracker else {
        return false;
    };

    chunk_batch_tracker.is_batch_running()
}

pub fn run_if_no_chunk_batch_running(chunk_batch_tracker: Option<Res<ChunkBatchTracker>>) -> bool {
    !run_if_chunk_batch_running(chunk_batch_tracker)
}
