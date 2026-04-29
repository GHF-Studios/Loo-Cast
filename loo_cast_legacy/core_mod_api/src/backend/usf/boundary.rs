use crate::bevy::prelude::*;
use crate::chunk::messages::{ChunkBoundaryDeltaKind, ChunkBoundaryDeltaMessage};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfChunkBoundaryMessageState {
    pub last_kind: Option<ChunkBoundaryDeltaKind>,
    pub last_batch_id: Option<u64>,
    pub last_spawn_count: usize,
    pub last_despawn_count: usize,
    pub total_messages: u64,
    pub total_spawn_targets_seen: u64,
    pub total_despawn_targets_seen: u64,
    pub active_scale: Scale,
    pub loader_origin_grid: GridVec,
    pub loader_origin_unit: UnitVec,
}
impl Default for UsfChunkBoundaryMessageState {
    fn default() -> Self {
        Self {
            last_kind: None,
            last_batch_id: None,
            last_spawn_count: 0,
            last_despawn_count: 0,
            total_messages: 0,
            total_spawn_targets_seen: 0,
            total_despawn_targets_seen: 0,
            active_scale: Scale::MAX,
            loader_origin_grid: GridVec::default(),
            loader_origin_unit: UnitVec::default(),
        }
    }
}

pub(crate) fn consume_chunk_boundary_delta_messages_system(
    mut messages: MessageReader<ChunkBoundaryDeltaMessage>,
    mut boundary_state: ResMut<UsfChunkBoundaryMessageState>,
) {
    for message in messages.read() {
        boundary_state.total_messages = boundary_state.total_messages.saturating_add(1);
        boundary_state.total_spawn_targets_seen = boundary_state
            .total_spawn_targets_seen
            .saturating_add(message.spawn_targets.len() as u64);
        boundary_state.total_despawn_targets_seen = boundary_state
            .total_despawn_targets_seen
            .saturating_add(message.despawn_targets.len() as u64);
        boundary_state.last_kind = Some(message.kind);
        boundary_state.last_batch_id = message.batch_id;
        boundary_state.last_spawn_count = message.spawn_targets.len();
        boundary_state.last_despawn_count = message.despawn_targets.len();
        boundary_state.active_scale = message.active_scale;
        boundary_state.loader_origin_grid = message.loader_origin_grid.clone();
        boundary_state.loader_origin_unit = message.loader_origin_unit.clone();
    }
}
