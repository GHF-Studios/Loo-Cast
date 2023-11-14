// Modules

// Local imports

// Internal imports
use crate::universe::chunk::id::*;

// External imports

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Eq, Hash, Debug, Clone)]
pub struct EntityID {
    chunk_id: ChunkID,
    local_id: u64,
}

// Implementations
impl PartialEq for EntityID {
    fn eq(&self, other: &Self) -> bool {
        self.chunk_id == other.chunk_id && self.local_id == other.local_id
    }
}

impl Default for EntityID {
    fn default() -> Self {
        EntityID {
            chunk_id: ChunkID::default(),
            local_id: 0,
        }
    }
}

impl EntityID {
    pub fn new(chunk_id: ChunkID, local_id: u64) -> Result<Self, String> {
        if local_id == u64::MAX {
            return Err("Cannot create entity id: Local id space has been exhausted.".to_string());
        }

        Ok(EntityID { chunk_id, local_id })
    }

    pub fn get_chunk_id(&self) -> &ChunkID {
        return &self.chunk_id;
    }

    pub fn get_local_id(&self) -> &u64 {
        return &self.local_id;
    }
}

// Module Functions
