use crate::bevy::prelude::*;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum ChunkBoundaryDeltaKind {
    Planned,
    Replanned,
    Cleared,
}

#[derive(Message, Clone, Debug, Reflect)]
pub struct ChunkBoundaryDeltaMessage {
    pub kind: ChunkBoundaryDeltaKind,
    pub batch_id: Option<u64>,
    pub spawn_targets: Vec<GridVec>,
    pub despawn_targets: Vec<GridVec>,
    pub active_scale: Scale,
    pub loader_origin_grid: GridVec,
    pub loader_origin_unit: UnitVec,
}
