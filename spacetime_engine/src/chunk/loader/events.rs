use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use super::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunkLoaderEntity {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub world_position: Vec2
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkLoaderEntity {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkLoaderEntity {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub target_entity_id: EntityID,
}

#[derive(Debug, Clone, Event)]
pub enum StartedChunkLoader {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID
    }
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkLoaderEntity {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
        world_position: Vec2
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        world_position: Vec2
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkLoaderEntity {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID
    }
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkLoaderEntity {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        target_entity_id: EntityID,
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        target_entity_id: EntityID
    }
}

#[derive(Debug, Clone, Event)]
pub struct CreateChunkLoaderEntityInternal {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: ChunkLoaderID,
    pub chunk_loader_entity_id: EntityID,
    pub world_position: Vec2
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkLoaderEntityInternal {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkLoaderEntityInternal {
    pub chunk_loader_request_id: ChunkLoaderRequestID,
    pub chunk_loader_id: ChunkLoaderID,
    pub target_entity_id: EntityID,
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkLoaderEntityInternal {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
        world_position: Vec2
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        world_position: Vec2
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkLoaderEntityInternal {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID
    }
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkLoaderEntityInternal {
    Success {
        chunk_loader_request_id: ChunkLoaderRequestID,
        chunk_loader_id: ChunkLoaderID,
        target_entity_id: EntityID,
    },
    Failure {
        chunk_loader_request_id: ChunkLoaderRequestID,
        target_entity_id: EntityID
    }
}