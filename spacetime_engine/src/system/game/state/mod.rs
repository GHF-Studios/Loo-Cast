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
    pub static ref GAME_STATE_MANAGER: Arc<Mutex<GameStateManager>> =
        Arc::new(Mutex::new(GameStateManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
pub struct GameState {

}

pub struct GameStateManager {
    manager_state: ManagerState,
    current_game_state: Option<GameState>,
}

// Implementations
impl Manager for GameStateManager {
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

impl GameStateManager {
    fn new() -> GameStateManager {
        GameStateManager {
            manager_state: ManagerState::Created,
            current_game_state: None,
        }
    }

    pub(in crate::system::game) fn load_game_state() {
        let game_state_manager = GAME_STATE_MANAGER.clone();
        let mut game_state_manager = match game_state_manager.lock() {
            Ok(game_state_manager) => {
                trace!("Successfully locked game state manager mutex.");
                game_state_manager
            },
            Err(_) => panic!("Failed to lock game state manager mutex!"),
        };

        match game_state_manager.current_game_state {
            None => {
                game_state_manager.current_game_state = Some(GameState {});
            }
            Some(_) => {
                error!("Game state already loaded!");
                return;
            }
        }
    }

    pub(in crate::system::game) fn unload_game_state() {
        let game_state_manager = GAME_STATE_MANAGER.clone();
        let mut game_state_manager = match game_state_manager.lock() {
            Ok(game_state_manager) => {
                trace!("Successfully locked game state manager mutex.");
                game_state_manager
            },
            Err(_) => panic!("Failed to lock game state manager mutex!"),
        };

        match game_state_manager.current_game_state {
            None => {
                error!("Game state already unloaded!");
                return;
            }
            Some(_) => {
                game_state_manager.current_game_state = None;
            }
        }
    }
}

// Module Functions
