// Modules

// Local imports

// Internal imports
use crate::system::universe::chunk::*;

// External imports
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone)]
pub struct EntityMetadata {
    pub(in crate::system::universe) parent_chunk: Arc<Mutex<Chunk>>,
}

// Implementations
impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            parent_chunk: Arc::new(Mutex::new(Chunk::default())),
        }
    }
}

impl EntityMetadata {
    pub fn new(parent_chunk: Arc<Mutex<Chunk>>) -> EntityMetadata {
        EntityMetadata { parent_chunk }
    }

    pub fn get_parent_chunk(&self) -> Arc<Mutex<Chunk>> {
        self.parent_chunk.clone()
    }
}

// Module Functions
