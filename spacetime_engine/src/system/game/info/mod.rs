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

    pub(in crate::system::game) fn handle_create_game_info(
        mut create_game_info_event_reader: EventReader<CreateGame>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
        for event in create_game_info_event_reader.iter() {
            let game_info_manager = GAME_INFO_MANAGER.clone();
            let mut game_info_manager = match game_info_manager.lock() {
                Ok(game_info_manager) => {
                    trace!("Locked game info manager mutex.");
                    game_info_manager
                },
                Err(_) => panic!("Failed to lock game info manager mutex!"),
            };

            let mut registered_game_infos = match &mut game_info_manager.registered_game_infos {
                None => {
                    error!("Game infos not registered!");
                    return;
                }
                Some(mut registered_game_infos) => registered_game_infos
            };

            let game_info: GameInfo = GameInfo {
                name: event.game_name.to_string(),
            };

            let serialized_game_info: String = serde_json::to_string(&game_info).unwrap();

            let dir_path = format!("mods/loo_cast_base_mod/data/games/{}", event.game_name);
            if !Path::new(&dir_path).exists() {
                std::fs::create_dir_all(&dir_path).expect("Failed to create save game directory");
            }

            let string_path = format!("{}/info.json", dir_path);
            let path = Path::new(&string_path);
            let display = path.display();

            let mut file = match File::create(path) {
                Err(why) => panic!("Couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            match file.write_all(serialized_game_info.as_bytes()) {
                Err(why) => panic!("Couldn't write to {}: {}", display, why),
                Ok(_) => println!("Wrote to {}", display),
            }

            registered_game_infos.push(game_info);

            app_state_next_state.set(AppState::GamesMenu);
        }
    }

    pub(in crate::system::game) fn handle_delete_game_info(
        mut delete_game_event_reader: EventReader<DeleteGame>,
        mut delete_game_ui_event_writer: EventWriter<DeleteGameUI>,
    ) {
        for event in delete_game_event_reader.iter() {
            let game_info_manager = GAME_INFO_MANAGER.clone();
            let mut game_info_manager = match game_info_manager.lock() {
                Ok(game_info_manager) => {
                    trace!("Locked game info manager mutex.");
                    game_info_manager
                },
                Err(_) => panic!("Failed to lock game info manager mutex!"),
            };

            let mut registered_game_infos = match &mut game_info_manager.registered_game_infos {
                None => {
                    error!("Game infos not registered!");
                    return;
                }
                Some(mut registered_game_infos) => registered_game_infos
            };
            
            let dir_path = format!("mods/loo_cast_base_mod/data/saves/{}", event.game_name);
            let string_path = format!("{}/info.json", dir_path);
            let path = Path::new(&string_path);
            let display = path.display();

            match std::fs::remove_file(path) {
                Err(why) => panic!("Couldn't delete {}: {}", display, why),
                Ok(_) => println!("Deleted {}", display),
            }

            std::fs::remove_dir_all(&dir_path).expect("Failed to remove save game directory");

            let mut index_to_remove: Option<usize> = None;
            for (index, game_info) in
            registered_game_infos.iter().enumerate()
            {
                if game_info.name == event.game_name {
                    index_to_remove = Some(index);
                    break;
                }
            }

            if let Some(index) = index_to_remove {
                registered_game_infos.remove(index);
            }

            delete_game_ui_event_writer.send(DeleteGameUI {
                game_name: event.game_name.to_string(),
            });
        }
    }
}

// Module Functions
