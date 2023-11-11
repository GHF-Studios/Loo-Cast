// Modules

// Local imports

// Internal imports
use crate::universe::chunk::*;
use crate::universe::entity::pos::*;

// External imports
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone)]
pub struct EntityMetadata {
    pub(super) pos: EntityPos,
    pub(super) parent_chunk: Arc<Mutex<Chunk>>,
}

// Implementations
impl EntityMetadata {
    pub fn new(
        parent_chunk: Arc<Mutex<Chunk>>,
        local_entity_pos: LocalEntityPos,
    ) -> Result<EntityMetadata, String> {
        let parent_chunk_temp = parent_chunk.lock().unwrap();
        let parent_chunk_metadata = match ChunkManager::get_metadata(&*parent_chunk_temp) {
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
