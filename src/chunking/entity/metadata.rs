// Modules


// Local imports


// Internal imports


// External imports
use std::sync::{Arc, Mutex};

// Static variables


// Constant variables


// Types


// Enums


// Structs
pub struct EntityMetadata {
    parent_chunk: Arc<Mutex<Chunk>>,
}

// Implementations
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

// Module Functions
