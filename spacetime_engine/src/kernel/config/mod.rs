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
    state: ManagerState,
}

// Implementations
impl Manager for ConfigManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        self.state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        self.state = ManagerState::Finalized;

        Ok(())
    }

    fn get_state(&self) -> &ManagerState {
        &self.state
    }
}

impl ConfigManager {
    fn new() -> ConfigManager {
        ConfigManager {
            state: ManagerState::Created,
        }
    }
}

// Module Functions
