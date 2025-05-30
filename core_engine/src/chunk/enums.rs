use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkAction {
    Spawn {
        requester_id: u32,
        coord: (i32, i32),
        new_owner: Option<Entity>,
        priority: ChunkActionPriority,
    },
    Despawn {
        requester_id: u32,
        coord: (i32, i32),
        priority: ChunkActionPriority,
    },
    TransferOwnership {
        requester_id: u32,
        coord: (i32, i32),
        new_owner: Entity,
        priority: ChunkActionPriority,
    },
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

    pub fn get_requester_id(&self) -> u32 {
        match self {
            ChunkAction::Spawn { requester_id, .. } | ChunkAction::Despawn { requester_id, .. } | ChunkAction::TransferOwnership { requester_id, .. } => {
                *requester_id
            }
        }
    }

    pub fn get_coord(&self) -> (i32, i32) {
        match self {
            ChunkAction::Spawn { coord, .. } | ChunkAction::Despawn { coord, .. } | ChunkAction::TransferOwnership { coord, .. } => *coord,
        }
    }

    pub fn get_priority(&self) -> ChunkActionPriority {
        match self {
            ChunkAction::Spawn { priority, .. } => *priority,
            ChunkAction::Despawn { priority, .. } => *priority,
            ChunkAction::TransferOwnership { priority, .. } => *priority,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkActionPriority {
    Deferred(i64),
    Realtime,
}

impl PartialOrd for ChunkActionPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChunkActionPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ChunkActionPriority::Realtime, ChunkActionPriority::Realtime) => std::cmp::Ordering::Equal,
            (ChunkActionPriority::Realtime, _) => std::cmp::Ordering::Greater,
            (_, ChunkActionPriority::Realtime) => std::cmp::Ordering::Less,
            (ChunkActionPriority::Deferred(a), ChunkActionPriority::Deferred(b)) => b.cmp(a),
        }
    }
}

impl Default for ChunkActionPriority {
    fn default() -> Self {
        ChunkActionPriority::Deferred(0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkState {
    Spawning,
    Loaded,
    Despawning,
}
