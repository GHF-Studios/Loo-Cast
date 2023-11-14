// Modules

// Local imports

// Internal imports
use crate::universe::*;
use crate::universe::chunk::*;
use crate::universe::entity::pos::*;

// External imports
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub(in crate::universe) pos: EntityPos,
    pub(in crate::universe) parent_chunk: Arc<Mutex<Chunk>>,
}

// Implementations
impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            pos: EntityPos::default(),
            parent_chunk: Arc::new(Mutex::new(Chunk::default())),
        }
    }
}

impl EntityMetadata {
    pub fn new(
        parent_chunk: Arc<Mutex<Chunk>>,
        local_entity_pos: LocalEntityPos,
    ) -> Result<EntityMetadata, String> {
        let parent_chunk_temp = parent_chunk.lock().unwrap();
        let parent_chunk_metadata = match UniverseManager::get_chunk_metadata(&*parent_chunk_temp) {
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
            pos: EntityPos::new(parent_chunk_pos, local_entity_pos),
            parent_chunk,
        })
    }

    pub fn get_parent_chunk(&self) -> Arc<Mutex<Chunk>> {
        return self.parent_chunk.clone();
    }
}

// Module Functions
