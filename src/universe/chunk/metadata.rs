// Modules

// Local imports

// Internal imports
use crate::universe::chunk::pos::*;
use crate::universe::chunk::*;
use crate::universe::entity::id::*;

// External imports
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Debug, Clone)]
pub struct ChunkMetadata {
    pub(super) pos: ChunkPos,
    pub(super) parent_chunk: Option<Arc<Mutex<Chunk>>>,
    pub(super) child_chunks: Option<HashMap<LocalChunkPos, Arc<Mutex<Chunk>>>>,
    pub(super) current_local_entity_id: u64,
    pub(super) recycled_local_entity_ids: Vec<u64>,
    pub(super) registered_entities: HashMap<EntityID, Arc<Mutex<crate::universe::entity::Entity>>>,
}

// Implementations
impl Default for ChunkMetadata {
    fn default() -> Self {
        Self {
            pos: ChunkPos::new(None, LocalChunkPos::new(0, 0)),
            parent_chunk: None,
            child_chunks: None,
            current_local_entity_id: 0,
            recycled_local_entity_ids: Vec::new(),
            registered_entities: HashMap::new(),
        }
    }
}

impl ChunkMetadata {
    pub fn new(
        parent_chunk: Option<Arc<Mutex<Chunk>>>,
        local_chunk_pos: LocalChunkPos,
    ) -> Result<ChunkMetadata, String> {
        if let Some(parent_chunk_mutex) = parent_chunk {
            let parent_chunk = parent_chunk_mutex.lock().unwrap();
            let parent_scale_index = ChunkManager::get_id(&*parent_chunk)
                .get_scale_index()
                .clone();
            let parent_chunk_metadata = match ChunkManager::get_metadata(&*parent_chunk) {
                Ok(parent_chunk_metadata) => parent_chunk_metadata,
                Err(error) => {
                    return Err(format!("Failed to get parent chunk metadata: {}", error))
                }
            };
            let parent_chunk_pos = parent_chunk_metadata.get_pos();
            drop(parent_chunk);

            if parent_scale_index < 62 {
                return Ok(ChunkMetadata {
                    pos: ChunkPos::new(Some(Box::new(parent_chunk_pos)), local_chunk_pos),
                    parent_chunk: Some(parent_chunk_mutex),
                    child_chunks: Some(HashMap::new()),
                    current_local_entity_id: 0,
                    recycled_local_entity_ids: Vec::new(),
                    registered_entities: HashMap::new(),
                });
            } else if parent_scale_index == 62 {
                return Ok(ChunkMetadata {
                    pos: ChunkPos::new(Some(Box::new(parent_chunk_pos)), local_chunk_pos),
                    parent_chunk: Some(parent_chunk_mutex),
                    child_chunks: None,
                    current_local_entity_id: 0,
                    recycled_local_entity_ids: Vec::new(),
                    registered_entities: HashMap::new(),
                });
            } else {
                return Err("Cannot create chunk with a scale index higher than 63".to_string());
            }
        } else {
            return Ok(ChunkMetadata {
                pos: ChunkPos::new(None, local_chunk_pos),
                parent_chunk: None,
                child_chunks: Some(HashMap::new()),
                current_local_entity_id: 0,
                recycled_local_entity_ids: Vec::new(),
                registered_entities: HashMap::new(),
            });
        }
    }

    pub fn get_parent_chunk(&self) -> Option<Arc<Mutex<Chunk>>> {
        return self.parent_chunk.clone();
    }

    pub fn get_pos(&self) -> ChunkPos {
        return self.pos.clone();
    }
}

// Module Functions
