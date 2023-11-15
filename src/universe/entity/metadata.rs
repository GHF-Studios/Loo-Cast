// Modules

// Local imports

// Internal imports
use crate::universe::*;
use crate::universe::chunk::*;
use crate::universe::entity::pos::*;
use crate::universe::local::*;

// External imports
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub(in crate::universe) parent_local_universe: Arc<Mutex<LocalUniverse>>,
    pub(in crate::universe) parent_chunk: Arc<Mutex<Chunk>>,
    pub(in crate::universe) pos: EntityPos,
}

// Implementations
impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            parent_local_universe: Arc::new(Mutex::new(LocalUniverse::default())),
            parent_chunk: Arc::new(Mutex::new(Chunk::default())),
            pos: EntityPos::default(),
        }
    }
}

impl EntityMetadata {
    pub fn new(
        parent_local_universe: Arc<Mutex<LocalUniverse>>,
        parent_chunk: Arc<Mutex<Chunk>>,
        local_entity_pos: LocalEntityPos,
    ) -> Result<EntityMetadata, String> {
        let parent_chunk_temp = parent_chunk.lock().unwrap();
        let parent_chunk_metadata = match GlobalUniverse::get_chunk_metadata(&*parent_chunk_temp) {
            Ok(metadata) => metadata,
            Err(e) => {
                return Err(format!(
                    "Failed to get the metadata of the parent chunk: {}",
                    e
                ))
            }
        };
        let parent_chunk_pos = parent_chunk_metadata.get_pos();
        drop(parent_chunk_temp);

        Ok(EntityMetadata {
            parent_local_universe,
            parent_chunk,
            pos: EntityPos::new(parent_chunk_pos, local_entity_pos),
        })
    }

    pub fn get_parent_local_universe(&self) -> Arc<Mutex<LocalUniverse>> {
        return self.parent_local_universe.clone();
    }

    pub fn get_parent_chunk(&self) -> Arc<Mutex<Chunk>> {
        return self.parent_chunk.clone();
    }

    pub fn get_pos(&self) -> EntityPos {
        return self.pos.clone();
    }
}

// Module Functions
