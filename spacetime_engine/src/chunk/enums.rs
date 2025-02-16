use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate) enum ChunkWorkflow {
    Spawn {
        requester_id: u32,
        coord: (i32, i32),
        new_owner: Option<Entity>,
        priority: ChunkWorkflowPriority,
    },
    Despawn {
        requester_id: u32,
        coord: (i32, i32),
        priority: ChunkWorkflowPriority,
    },
    TransferOwnership {
        requester_id: u32,
        coord: (i32, i32),
        new_owner: Entity,
        priority: ChunkWorkflowPriority,
    }
}
impl ChunkWorkflow {
    pub fn is_spawn(&self) -> bool {
        matches!(self, ChunkWorkflow::Spawn { .. })
    }

    pub fn is_despawn(&self) -> bool {
        matches!(self, ChunkWorkflow::Despawn { .. })
    }

    pub fn is_transfer_ownership(&self) -> bool {
        matches!(self, ChunkWorkflow::TransferOwnership { .. })
    }

    pub fn get_requester_id(&self) -> u32 {
        match self {
            ChunkWorkflow::Spawn { requester_id, .. }
            | ChunkWorkflow::Despawn { requester_id, .. }
            | ChunkWorkflow::TransferOwnership { requester_id, .. } => *requester_id
        }
    }

    pub fn get_coord(&self) -> (i32, i32) {
        match self {
            ChunkWorkflow::Spawn { coord, .. }
            | ChunkWorkflow::Despawn { coord, .. }
            | ChunkWorkflow::TransferOwnership { coord, .. } => *coord
        }
    }

    pub fn get_priority(&self) -> ChunkWorkflowPriority {
        match self {
            ChunkWorkflow::Spawn { priority, .. } => *priority,
            ChunkWorkflow::Despawn { priority, .. } => *priority,
            ChunkWorkflow::TransferOwnership { priority, .. } => *priority
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkWorkflowPriority {
    Deferred(i64),
    Realtime,
}

impl PartialOrd for ChunkWorkflowPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChunkWorkflowPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ChunkWorkflowPriority::Realtime, ChunkWorkflowPriority::Realtime) => std::cmp::Ordering::Equal,
            (ChunkWorkflowPriority::Realtime, _) => std::cmp::Ordering::Greater,
            (_, ChunkWorkflowPriority::Realtime) => std::cmp::Ordering::Less,
            (ChunkWorkflowPriority::Deferred(a), ChunkWorkflowPriority::Deferred(b)) => b.cmp(a),
        }
    }
}

impl Default for ChunkWorkflowPriority {
    fn default() -> Self {
        ChunkWorkflowPriority::Deferred(0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkState {
    Spawning,
    Loaded,
    Despawning,
}