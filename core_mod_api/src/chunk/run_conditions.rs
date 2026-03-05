use crate::bevy::prelude::*;

use crate::chunk::resources::ChunkLoadGate;

pub fn run_if_chunk_load_gate_open(chunk_load_gate: Option<Res<ChunkLoadGate>>) -> bool {
    let Some(chunk_load_gate) = chunk_load_gate else {
        return true;
    };

    !chunk_load_gate.is_locked()
}
