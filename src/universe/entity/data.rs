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
pub enum EntityRunState {
    Despawned,
    Spawned { bevy_entity: Entity },
}

// Structs
#[derive(Debug, Clone)]
pub struct EntityData {
    pub(in crate::universe) run_state: EntityRunState,
}

// Implementations
impl Default for EntityData {
    fn default() -> Self {
        Self {
            run_state: EntityRunState::Despawned,
        }
    }
}

impl EntityData {
    pub fn new() -> EntityData {
        EntityData {
            run_state: EntityRunState::Despawned,
        }
    }
}

// Module Functions
