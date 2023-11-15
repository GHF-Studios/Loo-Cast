// Modules

// Local imports

// Internal imports

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkRunState {
    Despawned,
    Spawned { bevy_entity: Entity },
}

// Structs
#[derive(Debug, Clone)]
pub struct ChunkData {
    pub(in crate::universe) run_state: ChunkRunState,
}

// Implementations
impl Default for ChunkData {
    fn default() -> Self {
        Self {
            run_state: ChunkRunState::Despawned,
        }
    }
}

impl ChunkData {
    pub fn new() -> ChunkData {
        ChunkData {
            run_state: ChunkRunState::Despawned,
        }
    }
}

// Module Functions
