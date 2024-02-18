// Modules

// Local imports

// Internal imports
use crate::system::universe::chunk::pos::*;
use crate::system::universe::chunk::*;

// External imports
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Debug, Clone, Default)]
pub struct ChunkMetadata {
    pub parent_chunk: Option<Arc<Mutex<Chunk>>>,
    pub(in crate::system::universe) absolute_local_chunk_pos: AbsoluteLocalChunkPos,
}

// Implementations

impl ChunkMetadata {
    pub fn new_node(
        parent_chunk: Arc<Mutex<Chunk>>,
        absolute_local_chunk_pos: AbsoluteLocalChunkPos,
    ) -> ChunkMetadata {
        ChunkMetadata {
            parent_chunk: Some(parent_chunk),
            absolute_local_chunk_pos,
        }
    }

    pub fn new_root(absolute_local_chunk_pos: AbsoluteLocalChunkPos) -> ChunkMetadata {
        ChunkMetadata {
            parent_chunk: None,
            absolute_local_chunk_pos,
        }
    }
}

// Module Functions
