// Modules

// Local imports

// Internal imports
use crate::engine::kernel::universe::chunk::pos::*;
use crate::engine::kernel::universe::chunk::*;
use crate::engine::kernel::universe::entity::id::*;

// External imports
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkRunState {
    Despawned,
    Spawned,
}

// Structs
#[derive(Debug, Clone)]
pub struct ChunkData {
    pub(in crate::engine::kernel::universe) run_state: ChunkRunState,
    pub(in crate::engine::kernel::universe) child_chunks: Option<HashMap<LocalChunkID, Arc<Mutex<Chunk>>>>,
    pub(in crate::engine::kernel::universe) current_local_entity_id: u64,
    pub(in crate::engine::kernel::universe) recycled_local_entity_ids: Vec<u64>,
    pub(in crate::engine::kernel::universe) registered_entities: HashMap<LocalEntityID, Arc<Mutex<entity::Entity>>>,
    pub(in crate::engine::kernel::universe) apparent_chunk_pos_shift: ApparentChunkPosShift,
}

// Implementations
impl Default for ChunkData {
    fn default() -> Self {
        Self {
            run_state: ChunkRunState::Despawned,
            child_chunks: None,
            current_local_entity_id: 0,
            recycled_local_entity_ids: Vec::new(),
            registered_entities: HashMap::new(),
            apparent_chunk_pos_shift: ApparentChunkPosShift::default(),
        }
    }
}

impl ChunkData {
    pub fn new_node(apparent_chunk_pos_shift: ApparentChunkPosShift) -> ChunkData {
        ChunkData {
            run_state: ChunkRunState::Despawned,
            child_chunks: Some(HashMap::new()),
            current_local_entity_id: 0,
            recycled_local_entity_ids: Vec::new(),
            registered_entities: HashMap::new(),
            apparent_chunk_pos_shift
        }
    }

    pub fn new_leaf(apparent_chunk_pos_shift: ApparentChunkPosShift) -> ChunkData {
        ChunkData {
            run_state: ChunkRunState::Despawned,
            child_chunks: None,
            current_local_entity_id: 0,
            recycled_local_entity_ids: Vec::new(),
            registered_entities: HashMap::new(),
            apparent_chunk_pos_shift
        }
    }
}

// Module Functions
