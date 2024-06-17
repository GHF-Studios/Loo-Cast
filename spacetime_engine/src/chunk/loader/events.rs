use bevy::prelude::*;
use crate::entity::id::structs::EntityID;
use super::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunkLoaderEntity {
    pub chunk_loader_event_id: ChunkLoaderEventID,
    pub world_position: Vec2
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkLoaderEntity {
    pub chunk_loader_event_id: ChunkLoaderEventID,
    pub chunk_loader_id: ChunkLoaderID
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkLoaderEntity {
    pub chunk_loader_event_id: ChunkLoaderEventID,
    pub target_entity_id: EntityID,
}

#[derive(Debug, Clone, Event)]
pub enum StartedChunkLoader {
    Success {
        chunk_loader_event_id: ChunkLoaderEventID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_event_id: ChunkLoaderEventID,
        chunk_loader_id: ChunkLoaderID
    }
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkLoaderEntity {
    Success {
        chunk_loader_event_id: ChunkLoaderEventID,
        chunk_loader_id: ChunkLoaderID,
        chunk_loader_entity_id: EntityID,
        world_position: Vec2
    },
    Failure {
        chunk_loader_event_id: ChunkLoaderEventID,
        world_position: Vec2
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkLoaderEntity {
    Success {
        chunk_loader_event_id: ChunkLoaderEventID,
        chunk_loader_id: ChunkLoaderID
    },
    Failure {
        chunk_loader_event_id: ChunkLoaderEventID,
        chunk_loader_id: ChunkLoaderID
    }
}

#[derive(Debug, Clone, Event)]
pub enum UpgradedToChunkLoaderEntity {
    Success {
        chunk_loader_event_id: ChunkLoaderEventID,
        chunk_loader_id: ChunkLoaderID,
        target_entity_id: EntityID,
    },
    Failure {
        chunk_loader_event_id: ChunkLoaderEventID,
        target_entity_id: EntityID
    }
}