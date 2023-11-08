// Modules
pub mod data;
pub mod id;
pub mod metadata;

// Local imports
use data::*;
use id::*;
use metadata::*;

// Internal imports
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


// Implementations
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

    pub fn get_id(&self) -> Arc<RwLock<EntityID>> {
        match self {
            Entity::Registered { id } => id.clone(),
            Entity::MetadataLoaded { id, .. } => id.clone(),
            Entity::DataLoaded { id, .. } => id.clone(),
        }
    }

    pub fn get_metadata(&self) -> Result<Arc<Mutex<EntityMetadata>>, String> {
        match self {
            Entity::Registered { .. } => Err("No metadata is loaded.".to_string()),
            Entity::MetadataLoaded { metadata, .. } => Ok(metadata.clone()),
            Entity::DataLoaded { metadata, .. } => Ok(metadata.clone()),
        }
    }

    pub fn get_data(&self) -> Result<Arc<Mutex<EntityData>>, String> {
        match self {
            Entity::Registered { .. } => Err("No data is loaded.".to_string()),
            Entity::MetadataLoaded { .. } => Err("No data is loaded.".to_string()),
            Entity::DataLoaded { data, .. } => Ok(data.clone()),
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
