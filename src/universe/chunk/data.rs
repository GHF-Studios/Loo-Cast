// Modules


// Local imports


// Internal imports


// External imports
use bevy::prelude::*;

// Static variables


// Constant variables


// Types


// Enums
#[derive(PartialEq)]
pub enum ChunkRunState {
    Despawned,
    Spawned {
        ecs_entity: Entity
    }
}

// Structs
pub struct ChunkData {
    pub(super) run_state: ChunkRunState,
}

// Implementations
impl ChunkData {
    pub fn new() -> ChunkData {
        ChunkData {
            run_state: ChunkRunState::Despawned,
        }
    }
}

// Module Functions
