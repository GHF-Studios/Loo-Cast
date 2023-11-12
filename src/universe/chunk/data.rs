// Modules

// Local imports

// Internal imports

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums
#[derive(Clone, PartialEq)]
pub enum ChunkRunState {
    Despawned,
    Spawned { ecs_entity: Entity },
}

// Structs
#[derive(Clone)]
pub struct ChunkData {
    pub(super) run_state: ChunkRunState,
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
