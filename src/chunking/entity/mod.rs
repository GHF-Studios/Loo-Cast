// Modules


// Local imports


// Internal imports
use crate::math::*;
use super::chunk::*;

// External imports
use std::sync::{Arc, Mutex, RwLock};

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
impl PartialEq for EntityID {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_id == other.chunk_id && self.local_id == other.local_id
    }
}

impl EntityID {
    pub fn new(chunk_id: ChunkID, local_id: u64) -> Result<Self, String> {
        if local_id == u64::MAX {
            return Err("Cannot create entity id: Local id space has been exhausted.".to_string());
        }

        Ok(EntityID {
            chunk_id,
            local_id,
        })
    }

    pub fn get_chunk_id(&self) -> ChunkID {
        return self.chunk_id;
    }

    pub fn get_local_id(&self) -> u64 {
        return self.local_id;
    }
}

impl EntityMetadata {
    pub fn new(parent_chunk: Arc<Mutex<Chunk>>) -> EntityMetadata {
        EntityMetadata {
            parent_chunk,
        }
    }

    pub fn get_parent_chunk(&self) -> Arc<Mutex<Chunk>> {
        return self.parent_chunk;
    }
}

impl EntityData {
    pub fn new() -> EntityData {
        EntityData {
            placeholder_data: None,
        }
    }

    pub fn get_placeholder_data(&self) -> Option<i32> {
        return self.placeholder_data;
    }

    pub fn set_placeholder_data(&mut self, placeholder_data: Option<i32>) {
        self.placeholder_data = placeholder_data;
    }
}

impl Entity {
    pub fn new(id: EntityID) -> Self {
        Entity::Registered {
            id: Arc::new(RwLock::new(id)),
        }
    }

    pub fn load_metadata(&mut self, metadata: EntityMetadata) -> Result<(), String> {
        match self {
            Entity::Registered { .. } => {
                *self = Entity::MetadataLoaded {
                    id: self.get_id().clone(),
                    metadata: Arc::new(Mutex::new(metadata)),
                };
                Ok(())
            },
            Entity::MetadataLoaded { .. } => {
                Err("Cannot load metadata: Metadata is already loaded.".to_string())
            }
            Entity::DataLoaded { .. } => {
                Err("Cannot load metadata: Both metadata and data are already loaded.".to_string())
            }
        }
    }

    pub fn load_data(&mut self, data: EntityData) -> Result<(), String> {
        match self {
            Entity::Registered { .. } => {
                Err("Cannot load data: Metadata must be loaded first.".to_string())
            }
            Entity::MetadataLoaded { .. } => {
                *self = Entity::DataLoaded {
                    id: self.get_id().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                    data: Arc::new(Mutex::new(data)),
                };
                Ok(())
            },
            Entity::DataLoaded { .. } => {
                Err("Cannot load data: Data is already loaded.".to_string())
            }
        }
    }

    pub fn unload_metadata(&mut self) -> Result<(), String> {
        match self {
            Entity::Registered { .. } => {
                Err("Cannot unload metadata: No metadata is loaded.".to_string())
            }
            Entity::MetadataLoaded { .. } => {
                *self = Entity::Registered {
                    id: self.get_id().clone(),
                };
                Ok(())
            }
            Entity::DataLoaded { .. } => {
                Err("Cannot unload metadata: Data must be unloaded first.".to_string())
            }
        }
    }

    pub fn unload_data(&mut self) -> Result<(), String> {
        match self {
            Entity::Registered { .. } => {
                Err("Cannot unload data: Neither metadata nor data are loaded.".to_string())
            }
            Entity::MetadataLoaded { .. } => {
                Err("Cannot unload data: No data is loaded.".to_string())
            }
            Entity::DataLoaded { .. } => {
                *self = Entity::MetadataLoaded {
                    id: self.get_id().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                };
                Ok(())
            }
        }
    }

    pub fn get_id(&self) -> &Arc<RwLock<EntityID>> {
        match self {
            Entity::Registered { id } => id,
            Entity::MetadataLoaded { id, .. } => id,
            Entity::DataLoaded { id, .. } => id,
        }
    }

    pub fn get_metadata(&self) -> Result<&Arc<Mutex<EntityMetadata>>, String> {
        match self {
            Entity::Registered { .. } => Err("No metadata is loaded.".to_string()),
            Entity::MetadataLoaded { metadata, .. } => Ok(metadata),
            Entity::DataLoaded { metadata, .. } => Ok(metadata),
        }
    }

    pub fn get_data(&self) -> Result<&Arc<Mutex<EntityData>>, String> {
        match self {
            Entity::Registered { .. } => Err("No data is loaded.".to_string()),
            Entity::MetadataLoaded { .. } => Err("No data is loaded.".to_string()),
            Entity::DataLoaded { data, .. } => Ok(data),
        }
    }

    pub fn get_load_state(&self) -> EntityLoadState {
        match self {
            Entity::Registered { .. } => EntityLoadState::Registered,
            Entity::MetadataLoaded { .. } => EntityLoadState::MetadataLoaded,
            Entity::DataLoaded { .. } => EntityLoadState::DataLoaded,
        }
    }
}

// Module Functions
