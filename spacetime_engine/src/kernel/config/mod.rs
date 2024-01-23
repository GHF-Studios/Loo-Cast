// Modules

// Local imports

// Internal imports
use super::manager::*;

// External imports
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref CONFIG_MANAGER: Arc<Mutex<ConfigManager>> =
        Arc::new(Mutex::new(ConfigManager::new()));
}

// Constant variables

// Types

// Traits

// Enums

// Structs
pub struct ConfigManager {
    manager_state: ManagerState,
}

// Implementations
impl Manager for ConfigManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.manager_state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Finalized;

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl ConfigManager {
    fn new() -> ConfigManager {
        ConfigManager {
            manager_state: ManagerState::Created,
        }
    }
}

// Module Functions
