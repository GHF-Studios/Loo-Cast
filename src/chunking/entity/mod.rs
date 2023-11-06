// Modules


// Local imports


// Internal imports
use crate::math::*;
use super::chunk::*;

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
struct LocalIDCounter {
    current_local_id: u64,
}

pub struct EntityID {
    chunk_id: ChunkID,
    local_id: u64,
    global_id_base10: BigUint,
    global_id_base10x10: Vec<(u8, u8)>,
    global_id_base57: String,
}

pub struct EntityMetadata {
    placeholder_metadata: Option<i32>,
}

pub struct EntityData {
    placeholder_data: Option<i32>,
}

pub struct EntityManager {
    registered_chunks: Arc<Mutex<Vec<Entity>>>,
    local_id_counter: LocalIDCounter,
}

// Implementations
impl LocalIDCounter {
    pub fn new(current_local_id: u64) -> Self {
        LocalIDCounter {
            current_local_id,
        }
    }

    pub fn get_unique_local_id(&mut self) -> u64 {
        let local_id = self.current_local_id;
        self.current_local_id += 1;
        local_id
    }
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            registered_chunks: Arc::new(Mutex::new(Vec::new())),
            local_id_counter: LocalIDCounter::new(0),
        }
    }
}

impl EntityID {
    pub fn get_chunk_id(&self) -> u8 {
        return self.chunk_id;
    }

    pub fn get_local_id(&self) -> u8 {
        return self.local_id;
    }

    pub fn get_global_id_base10(&self) -> &BigUint {
        return &self.global_id_base10;
    }

    pub fn get_global_id_base10x10(&self) -> &Vec<(u8, u8)> {
        return &self.global_id_base10x10;
    }

    pub fn get_global_id_base57(&self) -> &String {
        return &self.global_id_base57;
    }
}

// Module Functions
