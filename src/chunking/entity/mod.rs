// Modules


// Local imports


// Internal imports
use crate::math::*;
use super::chunk::*;
use super::identification::LocalID::*;

// External imports
use bevy::prelude::*;

// Static variables


// Constant variables


// Types


// Enums
pub enum Entity {
    Registered {
        id: Arc<RwLock<EntityID>>,
    },
    MetadataLoaded {
        id: Arc<RwLock<EntityID>>,
        metadata: Arc<Mutex<EntityMetadata>>,
    },
    DataLoaded {
        id: Arc<RwLock<EntityID>>,
        metadata: Arc<Mutex<EntityMetadata>>,
        data: Arc<Mutex<EntityData>>,
    },
}

pub enum EntityLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

// Structs
pub struct EntityID {
    chunk_id: ChunkID,
    local_id: u64,
}

pub struct EntityMetadata {
    parent_chunk: Arc<Mutex<Chunk>>,
}

pub struct EntityData {
    placeholder_data: Option<i32>,
}

// Implementations
impl EntityID {
    pub fn new(chunk_id: ChunkID, local_id: u64) -> Result<Self, String> {
        if self.local_id == u64::MAX {
            return Err("Cannot create entity id: Local id space has been exhausted.".to_string());
        }

        Ok(EntityID {
            chunk_id,
            local_id,
        })
    }

    pub fn get_chunk_id(&self) -> u8 {
        return self.chunk_id;
    }

    pub fn get_local_id(&self) -> u8 {
        return self.local_id;
    }
}

// Module Functions
