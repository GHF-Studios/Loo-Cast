use bevy::prelude::*;
use crate::chunk::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateChunkEntity {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct DestroyChunkEntity {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct LoadChunkEntity {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub struct UnloadChunkEntity {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub enum CreatedChunkEntity {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum DestroyedChunkEntity {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum LoadedChunkEntity {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub enum UnloadedChunkEntity {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct CreateChunkEntityInternal {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct DestroyChunkEntityInternal {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct LoadChunkEntityInternal {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) struct UnloadChunkEntityInternal {
    pub chunk_event_id: ChunkEventID,
    pub chunk_id: ChunkID,
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum CreatedChunkEntityInternal {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum DestroyedChunkEntityInternal {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum LoadedChunkEntityInternal {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}

#[derive(Debug, Clone, Event)]
pub(in crate) enum UnloadedChunkEntityInternal {
    Success {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    },
    Failure {
        chunk_event_id: ChunkEventID,
        chunk_id: ChunkID,
    }
}