use bevy::prelude::*;
use crate::chunk::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunkEntity {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkEntity {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct LoadChunkEntity {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct UnloadChunkEntity {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkEntity {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkEntity {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum LoadedChunkEntity {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum UnloadedChunkEntity {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct CreateChunkEntityInternal {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct DestroyChunkEntityInternal {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct LoadChunkEntityInternal {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct UnloadChunkEntityInternal {
    pub chunk_request_id: ChunkRequestID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum CreatedChunkEntityInternal {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum DestroyedChunkEntityInternal {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum LoadedChunkEntityInternal {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum UnloadedChunkEntityInternal {
    Success {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_request_id: ChunkRequestID,
        chunk_id: ChunkID,
    }
}