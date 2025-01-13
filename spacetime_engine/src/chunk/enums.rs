use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate) enum ChunkAction {
    Spawn {
        coord: (i32, i32),
        owner: Option<Entity>
    },
    Despawn {
        coord: (i32, i32)
    },
    TransferOwnership {
        coord: (i32, i32),
        new_owner: Entity
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
            ChunkAction::Spawn { coord, .. } => *coord,
            ChunkAction::Despawn { coord } => *coord,
            ChunkAction::TransferOwnership { coord, .. } => *coord
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkState {
    Spawning,
    Loaded,
    Despawning,
}