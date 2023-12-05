// Modules

// Local imports

// Internal imports
use crate::universe::chunk::pos::*;
use crate::universe::chunk::*;
use crate::universe::*;

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
    pub(in crate::universe) pos: ChunkPos,
    pub(in crate::universe) parent_chunk: Option<Arc<Mutex<Chunk>>>,
    pub(in crate::universe) child_chunks: Option<HashMap<AbsoluteLocalChunkPos, Arc<Mutex<Chunk>>>>,
    pub(in crate::universe) current_local_entity_id: u64,
    pub(in crate::universe) recycled_local_entity_ids: Vec<u64>,
    pub(in crate::universe) registered_entities: HashMap<LocalEntityID, Arc<Mutex<entity::Entity>>>,
}

// Implementations
impl Default for ChunkMetadata {
    fn default() -> Self {
        Self {
            pos: ChunkPos::from_absolute(None, AbsoluteLocalChunkPos::new(0, 0)),
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
        apparent_local_pos: ApparentLocalChunkPos,
    ) -> Result<ChunkMetadata, String> {
        if let Some(parent_chunk_mutex) = parent_chunk {
            let parent_chunk = parent_chunk_mutex.lock().unwrap();
            let parent_scale_index = GlobalUniverse::get_chunk_id(&*parent_chunk)
                .get_scale_index()
                .clone();
            let parent_chunk_metadata = match GlobalUniverse::get_chunk_metadata(&*parent_chunk) {
                Ok(parent_chunk_metadata) => parent_chunk_metadata,
                Err(error) => {
                    return Err(format!("Failed to get parent chunk metadata: {}", error))
                }
            };
            let parent_chunk_pos = parent_chunk_metadata.get_pos();
            drop(parent_chunk);

            if parent_scale_index < 62 {
                return Ok(ChunkMetadata {
                    pos: ChunkPos::from_apparent(Some(Box::new(parent_chunk_pos)), apparent_local_pos),
                    parent_chunk: Some(parent_chunk_mutex),
                    child_chunks: Some(HashMap::new()),
                    current_local_entity_id: 0,
                    recycled_local_entity_ids: Vec::new(),
                    registered_entities: HashMap::new(),
                });
            } else if parent_scale_index == 62 {
                return Ok(ChunkMetadata {
                    pos: ChunkPos::from_apparent(Some(Box::new(parent_chunk_pos)), apparent_local_pos),
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
                pos: ChunkPos::from_apparent(None, apparent_local_pos),
                parent_chunk: None,
                child_chunks: Some(HashMap::new()),
                current_local_entity_id: 0,
                recycled_local_entity_ids: Vec::new(),
                registered_entities: HashMap::new(),
            });
        }
    }

    pub fn get_pos(&self) -> ChunkPos {
        return self.pos.clone();
    }

    pub fn get_parent_chunk(&self) -> Option<Arc<Mutex<Chunk>>> {
        return self.parent_chunk.clone();
    }

    pub fn get_child_chunks(&self) -> &Option<HashMap<AbsoluteLocalChunkPos, Arc<Mutex<Chunk>>>> {
        return &self.child_chunks;
    }

    pub fn get_registered_entities(&self) -> &HashMap<LocalEntityID, Arc<Mutex<entity::Entity>>> {
        return &self.registered_entities;
    }
}

// Module Functions
