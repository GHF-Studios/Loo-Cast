// Modules


// Local imports


// Internal imports
use crate::chunking::chunk::pos::*;

// External imports
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Static variables


// Constant variables


// Types


// Enums


// Structs
pub struct ChunkMetadata {
    pos: ChunkPos,
    parent_chunk: Option<Arc<Mutex<Chunk>>>,
    child_chunks: Option<HashMap<ChunkID, Arc<Mutex<Chunk>>>>,
    current_local_entity_id: u64,
    recycled_local_entity_ids: Vec<u64>,
    registered_entities: Hashmap<EntityID, Arc<Mutex<crate::chunking::entity::Entity>>>,
}

// Implementations
impl ChunkMetadata {
    pub fn new(parent_chunk: Option<Arc<Mutex<Chunk>>>, local_chunk_pos: LocalChunkPos) -> Result<ChunkMetadata, String> {
        if let Some(parent_chunk) = parent_chunk {
            let parent_chunk_temp = parent_chunk.lock().unwrap();
            let parent_scale_index = parent_chunk.get_id().lock().unwrap().get_global_id_base10x10().len() - 1;
            let parent_chunk_pos = parent_chunk.get_metadata().lock().unwrap().get_pos();
            drop(parent_chunk_temp);

            if parent_scale_index < 63 {
                return Ok(ChunkMetadata {
                    pos: ChunkPos::new(Some(parent_chunk_pos), local_chunk_pos),
                    parent_chunk,
                    child_chunks: Some(HashMap::new()), 
                    current_local_entity_id: 0,
                    registered_entities: HashMap::new(),
                });
            } else if parent_scale_index == 63 {
                return Ok(ChunkMetadata {
                    pos: ChunkPos::new(Some(parent_chunk_pos), local_chunk_pos),
                    parent_chunk,
                    child_chunks: None, 
                    current_local_entity_id: 0,
                    registered_entities: HashMap::new(),
                });
            } else if parent_scale_index > 63 {
                return Err("Cannot create chunk with a scale index higher than 63".to_string());
            }
        } else {
            return Ok(ChunkMetadata {
                pos: ChunkPos::new(None, local_chunk_pos),
                parent_chunk: None,
                child_chunks: HashMap::new(), 
                current_local_entity_id: 0,
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
