use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use super::id::structs::*;

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub struct CreateChunkLoaderEntity {
    pub chunk_loader_id: ChunkLoaderID
}

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub struct DestroyChunkLoaderEntity {
    pub chunk_loader_id: ChunkLoaderID
}

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkLoaderEntity {
    pub target_entity_id: EntityID,
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub enum StartedChunkLoader {
    Success {
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub enum CreatedChunkLoaderEntity {
    Success {
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkLoaderEntity {
    Success {
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

// TODO: Implement
#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkLoaderEntity {
    Success {
        target_entity_id: EntityID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        target_entity_id: EntityID
    }
}