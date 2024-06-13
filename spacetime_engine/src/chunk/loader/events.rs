use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use super::id::structs::*;

// TODO: Implement
#[derive(Clone, Event)]
pub struct CreateChunkLoaderEntity {
    pub chunk_loader_id: ChunkLoaderID
}

// TODO: Implement
#[derive(Clone, Event)]
pub struct DestroyChunkLoaderEntity {
    pub chunk_loader_id: ChunkLoaderID
}

// TODO: Implement
#[derive(Clone, Event)]
pub struct UpgradeToChunkLoaderEntity {
    pub target_entity_id: EntityID,
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub enum StartChunkLoaderResult {
    Success {
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

// TODO: Implement
pub enum CreateChunkLoaderEntityResult {
    Success {
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

// TODO: Implement
pub enum DestroyChunkLoaderEntityResult {
    Success {
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_id: ChunkLoaderID
    }
}

// TODO: Implement
pub enum UpgradeToChunkLoaderEntityResult {
    Success {
        target_entity_id: EntityID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        target_entity_id: EntityID
    }
}