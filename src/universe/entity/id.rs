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
#[derive(Eq, Hash, Debug, Clone, Copy)]
pub struct LocalEntityID {
    id: u64,
}

#[derive(Eq, Hash, Debug, Clone)]
pub struct EntityID {
    parent_chunk_id: ChunkID,
    local_entity_id: LocalEntityID,
}

// Implementations
impl PartialEq for LocalEntityID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for LocalEntityID {
    fn default() -> Self {
        LocalEntityID { id: 0 }
    }
}

impl LocalEntityID {
    pub(in crate::universe) fn new(id: u64) -> Result<Self, String> {
        if id == u64::MAX {
            return Err("Cannot create local entity id: ID cannot be u64::MAX.".to_string());
        }

        Ok(LocalEntityID { id })
    }

    pub fn get_id(&self) -> u64 {
        return self.id;
    }
}

impl PartialEq for EntityID {
    fn eq(&self, other: &Self) -> bool {
        self.parent_chunk_id == other.parent_chunk_id && self.local_entity_id == other.local_entity_id
    }
}

impl Default for EntityID {
    fn default() -> Self {
        EntityID {
            parent_chunk_id: ChunkID::default(),
            local_entity_id: LocalEntityID::default(),
        }
    }
}

impl EntityID {
    pub(in crate::universe) fn new(
        parent_chunk_id: ChunkID,
        local_entity_id: LocalEntityID,
    ) -> Self {
        EntityID {
            parent_chunk_id,
            local_entity_id,
        }
    }

    pub fn get_parent_chunk_id(&self) -> &ChunkID {
        return &self.parent_chunk_id;
    }

    pub fn get_local_entity_id(&self) -> LocalEntityID {
        return self.local_entity_id;
    }
}

// Module Functions
