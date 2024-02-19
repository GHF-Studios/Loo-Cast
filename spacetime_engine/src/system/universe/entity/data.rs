// Modules

// Local imports

// Internal imports

// External imports

// Static variables

// Constant variables

// Types

// Enums
#[derive(Debug, Clone, PartialEq)]
pub enum EntityRunState {
    Despawned,
    Spawned,
}

// Structs
#[derive(Clone)]
pub struct EntityData {
    pub(in crate::system::universe) run_state: EntityRunState,
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

    pub fn get_run_state(&self) -> EntityRunState {
        self.run_state.clone()
    }
}

// Module Functions
