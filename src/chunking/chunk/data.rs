// Modules


// Local imports


// Internal imports


// External imports
use bevy::prelude::*;

// Static variables


// Constant variables


// Types


// Enums
pub enum ChunkRunState {
    Despawned,
    Spawned {
        ecs_entity: Entity
    }
}

// Structs
pub struct ChunkData {
    run_state: ChunkRunState,
}

// Implementations
impl ChunkData {
    pub fn new() -> ChunkData {
        ChunkData {
            run_state: ChunkRunState::Despawned,
        }
    }

    pub fn get_run_state(&self) -> ChunkRunState {
        return self.run_state;
    }

    pub fn set_run_state(&mut self, run_state: ChunkRunState) {
        self.run_state = run_state;
    }
}

// Module Functions
