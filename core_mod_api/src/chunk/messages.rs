use crate::bevy::prelude::*;

use super::resources::{ChunkBatchCancellationReason, ChunkBatchSnapshot};

#[derive(Message, Clone, Debug)]
pub enum ChunkBatchLifecycleMessage {
    Planned {
        batch: ChunkBatchSnapshot,
    },
    Started {
        batch: ChunkBatchSnapshot,
    },
    Finished {
        batch: ChunkBatchSnapshot,
    },
    Cancelled {
        batch_id: u64,
        reason: ChunkBatchCancellationReason,
        spawn_count: usize,
        despawn_count: usize,
    },
}
