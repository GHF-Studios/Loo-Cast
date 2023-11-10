// Modules


// Local imports


// Internal imports
use crate::universe::entity::pos::*;
use crate::universe::chunk::*;

// External imports
use std::sync::{Arc, Mutex};

// Static variables


// Constant variables


// Types


// Enums


// Structs
pub struct EntityMetadata {
    pub(super) pos: EntityPos,
    pub(super) parent_chunk: Arc<Mutex<Chunk>>,
}

// Implementations
impl EntityMetadata {
    pub fn new(parent_chunk: Arc<Mutex<Chunk>>, local_entity_pos: LocalEntityPos) -> EntityMetadata {
        let parent_chunk_temp = parent_chunk.lock().unwrap();
        let parent_chunk_pos = parent_chunk.get_metadata().lock().unwrap().get_pos();
        drop(parent_chunk_temp);

        EntityMetadata {
            pos: EntityPos::new(Some(parent_chunk_pos), local_entity_pos),
            parent_chunk,
        }
    }

    pub fn get_parent_chunk(&self) -> Arc<Mutex<Chunk>> {
        return self.parent_chunk.clone();
    }
}

// Module Functions
