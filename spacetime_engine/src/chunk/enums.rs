use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate) enum ChunkAction {
    Spawn {
        coord: (i32, i32),
        owner: Option<Entity>,
        priority: ChunkActionPriority,
    },
    Despawn {
        coord: (i32, i32),
        priority: ChunkActionPriority,
    },
    TransferOwnership {
        coord: (i32, i32),
        new_owner: Entity,
        priority: ChunkActionPriority,
    }
}
impl ChunkAction {
    pub fn is_spawn(&self) -> bool {
        matches!(self, ChunkAction::Spawn { .. })
    }

    pub fn is_despawn(&self) -> bool {
        matches!(self, ChunkAction::Despawn { .. })
    }

    pub fn is_transfer_ownership(&self) -> bool {
        matches!(self, ChunkAction::TransferOwnership { .. })
    }

    pub fn get_coord(&self) -> (i32, i32) {
        match self {
            ChunkAction::Spawn { coord, .. }
            | ChunkAction::Despawn { coord, .. }
            | ChunkAction::TransferOwnership { coord, .. } => *coord
        }
    }

    pub fn get_priority(&self) -> ChunkActionPriority {
        match self {
            ChunkAction::Spawn { priority, .. } => *priority,
            ChunkAction::Despawn { priority, .. } => *priority,
            ChunkAction::TransferOwnership { priority, .. } => *priority
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChunkActionPriority {
    Realtime,
    VeryHigh,
    High,
    #[default]
    Medium,
    Low,
    VeryLow,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkState {
    Spawning,
    Loaded,
    Despawning,
}