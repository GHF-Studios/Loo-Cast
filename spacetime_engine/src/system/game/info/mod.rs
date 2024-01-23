// Modules

// Local imports

// Internal imports
use super::{CreateGame, DeleteGame};
use crate::system::ui::games_menu::*;
use crate::system::AppState;
use crate::kernel::manager::*;

// External imports
use serde::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use lazy_static::*;
use std::sync::{Arc, Mutex};
use bevy::prelude::*;

// Static variables
lazy_static! {
    pub static ref GAME_INFO_MANAGER: Arc<Mutex<GameInfoManager>> =
        Arc::new(Mutex::new(GameInfoManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
#[derive(Serialize, Deserialize, Clone)]
pub struct GameInfo {
    pub name: String,
}

pub struct GameInfoManager {
    manager_state: ManagerState,
    registered_game_infos: Option<Vec<GameInfo>>,
}

// Implementations
impl Manager for GameInfoManager {
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

impl GameInfoManager {
    fn new() -> GameInfoManager {
        GameInfoManager {
            manager_state: ManagerState::Created,
            registered_game_infos: Some(Vec::new()),
        }
    }

    pub(in crate::system::game) fn register_game_info(&mut self, game_info: GameInfo) {
        match &mut self.registered_game_infos {
            None => {
                error!("Game infos not registered!");
                return;
            }
            Some(registered_game_infos) => {
                registered_game_infos.push(game_info);
            }
        }
    }

    pub(in crate::system::game) fn register_game_infos() {
        let game_info_manager = GAME_INFO_MANAGER.clone();
        let mut game_info_manager = match game_info_manager.lock() {
            Ok(game_info_manager) => {
                trace!("Locked game info manager mutex.");
                game_info_manager
            },
            Err(_) => panic!("Failed to lock game info manager mutex!"),
        };

        match game_info_manager.registered_game_infos {
            None => {}
            Some(_) => {
                error!("Game infos already loaded!");
                return;
            }
        }

        let mut game_infos: Vec<GameInfo> = Vec::new();

        if let Ok(paths) = std::fs::read_dir("loo_cast_base_mod/data/games") {
            for path in paths {
                let path = path.unwrap().path();
                if path.is_dir() {
                    let info_path = path.join("info.json");
                    let display = info_path.display();

                    let mut file = match File::open(&info_path) {
                        Err(why) => panic!("Couldn't open {}: {}", display, why),
                        Ok(file) => file,
                    };

                    let mut serialized_game_info = String::new();
                    match file.read_to_string(&mut serialized_game_info) {
                        Err(why) => panic!("Couldn't read {}: {}", display, why),
                        Ok(_) => println!("Read {}", display),
                    }

                    game_infos.push(serde_json::from_str(&serialized_game_info).unwrap());
                }
            }
        }

        game_info_manager.registered_game_infos = Some(game_infos);
    }

    pub fn get_game_info(&self, game_name: String) -> Option<&GameInfo> {
        match &self.registered_game_infos {
            None => {
                error!("Game infos not registered!");
                return None;
            }
            Some(registered_game_infos) => {
                for game_info in registered_game_infos {
                    if game_info.name == game_name {
                        return Some(game_info);
                    }
                }
            }
        }

        None
    }

    pub fn get_game_infos(&self) -> Option<&Vec<GameInfo>> {
        match &self.registered_game_infos {
            None => {
                error!("Game infos not registered!");
                return None;
            }
            Some(registered_game_infos) => Some(registered_game_infos),
        }
    }

    pub(in crate::system::game) fn get_game_infos_mut(&mut self) -> Option<&mut Vec<GameInfo>> {
        match &mut self.registered_game_infos {
            None => {
                error!("Game infos not registered!");
                return None;
            }
            Some(registered_game_infos) => Some(registered_game_infos),
        }
    }
}

// Module Functions
