// Modules

// Local imports

// Internal imports
use crate::kernel::manager::*;

// External imports
use bevy::prelude::*;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref GAME_CONFIG_MANAGER: Arc<Mutex<GameConfigManager>> =
        Arc::new(Mutex::new(GameConfigManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
pub struct GameConfig {

}

pub struct GameConfigManager {
    manager_state: ManagerState,
    current_game_config: Option<GameConfig>,
}

// Implementations
impl Manager for GameConfigManager {
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

impl GameConfigManager {
    fn new() -> GameConfigManager {
        GameConfigManager {
            manager_state: ManagerState::Created,
            current_game_config: None,
        }
    }

    pub(in crate::system::game) fn load_game_config() {
        let game_config_manager = GAME_CONFIG_MANAGER.clone();
        let mut game_config_manager = match game_config_manager.lock() {
            Ok(game_config_manager) => {
                trace!("Locked game config manager mutex.");
                game_config_manager
            },
            Err(_) => panic!("Failed to lock game config manager mutex!"),
        };

        match game_config_manager.current_game_config {
            None => {
                game_config_manager.current_game_config = Some(GameConfig {});
            }
            Some(_) => {
                error!("Game config already loaded!");
                return;
            }
        }
    }

    pub(in crate::system::game) fn unload_game_config() {
        let game_config_manager = GAME_CONFIG_MANAGER.clone();
        let mut game_config_manager = match game_config_manager.lock() {
            Ok(game_config_manager) => {
                trace!("Locked game config manager mutex.");
                game_config_manager
            },
            Err(_) => panic!("Failed to lock game config manager mutex!"),
        };

        match game_config_manager.current_game_config {
            None => {
                error!("Game config already unloaded!");
                return;
            }
            Some(_) => {
                game_config_manager.current_game_config = None;
            }
        }
    }
}

// Module Functions
