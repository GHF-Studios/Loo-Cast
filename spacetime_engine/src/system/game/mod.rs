// Modules
pub mod config;
pub mod info;
pub mod state;

// Local imports
use config::*;
use info::*;
use state::*;

// Internal imports
use crate::kernel::manager::*;
use crate::system::ui::games_menu::*;
use crate::system::universe::*;
use crate::system::AppState;

// External imports
use bevy::app::AppExit;
use bevy::prelude::*;
use core::panic;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref GAME_MANAGER: Arc<Mutex<GameManager>> =
        Arc::new(Mutex::new(GameManager::new()));
}

// Constant variables

// Types

// Enums
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LoadState {
    #[default]
    LoadedGame,
    LoadedGameConfig,
    LoadedGameState,
    LoadedUniverse,
    FullyLoaded,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameQuitMode {
    QuitToMainMenu,
    QuitToDesktop,
}

// Structs
pub struct GamePlugin;

#[derive(Event)]
pub struct LoadGame {
    pub game: GameInfo,
}

#[derive(Event)]
pub struct UnloadGame {
    pub quit_mode: GameQuitMode,
}

#[derive(Event)]
pub struct CreateGame {
    pub game_name: String,
}

#[derive(Event)]
pub struct DeleteGame {
    pub game_name: String,
}

#[derive(Resource)]
pub struct GameManager {
    manager_state: ManagerState,
    current_game: Option<GameInfo>,
}

// Implementations
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<CreateGame>()
            .add_event::<DeleteGame>()
            .add_event::<LoadGame>()
            .add_event::<UnloadGame>()
            // States
            .add_state::<SimulationState>()
            .add_state::<LoadState>()
            // Startup Systems
            .add_systems(Startup, GameInfoManager::register_game_infos)
            // Update Systems
            .add_systems(
                Update, (
                    GameManager::handle_load_game, 
                    GameManager::handle_delete_game
                ).run_if(in_state(AppState::GamesMenu)),
            )
            .add_systems(
                Update,
                (GameManager::handle_create_game)
                    .run_if(in_state(AppState::CreateGameInfoMenu)),
            )
            .add_systems(
                Update,
                (
                    GameManager::handle_toggle_simulation,
                    GameManager::handle_unload_game,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

impl Manager for GameManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        info!("Initializing game main module...");

        match self.manager_state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        debug!("Locking game module manager mutexes...");

        let game_info_manager = GAME_INFO_MANAGER.clone();
        let mut game_info_manager = match game_info_manager.lock() {
            Ok(game_info_manager) => {
                trace!("Locked game info manager mutex.");
                game_info_manager
            },
            Err(_) => panic!("Failed to lock game info manager mutex!"),
        };
        let game_config_manager = GAME_CONFIG_MANAGER.clone();
        let mut game_config_manager = match game_config_manager.lock() {
            Ok(game_config_manager) => {
                trace!("Locked game config manager mutex.");
                game_config_manager
            },
            Err(_) => panic!("Failed to lock game config manager mutex!"),
        };
        let game_state_manager = GAME_STATE_MANAGER.clone();
        let mut game_state_manager = match game_state_manager.lock() {
            Ok(game_state_manager) => {
                trace!("Locked game state manager mutex.");
                game_state_manager
            },
            Err(_) => panic!("Failed to lock game state manager mutex!"),
        };

        debug!("Locked game module manager mutexes.");

        info!("Initializing game main module....");

        match game_info_manager.initialize() {
            Ok(_) => {
                trace!("Initialized game info module.");
            }
            Err(_) => {
                panic!("Failed to initialize game info module!");
            }
        };
        match game_config_manager.initialize() {
            Ok(_) => {
                trace!("Initialized game config module.");
            }
            Err(_) => {
                panic!("Failed to initialize game config module!");
            }
        };
        match game_state_manager.initialize() {
            Ok(_) => {
                trace!("Initialized game state module.");
            }
            Err(_) => {
                panic!("Failed to initialize game state module!");
            }
        };

        info!("Initialized game main module..");

        self.manager_state = ManagerState::Initialized;

        info!("Initialized game main module.");

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        info!("Finalizing game main module...");

        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        debug!("Locking game module manager mutexes...");

        let game_state_manager = GAME_STATE_MANAGER.clone();
        let mut game_state_manager = match game_state_manager.lock() {
            Ok(game_state_manager) => {
                trace!("Locked game state manager mutex.");
                game_state_manager
            },
            Err(_) => panic!("Failed to lock game state manager mutex!"),
        };
        let game_config_manager = GAME_CONFIG_MANAGER.clone();
        let mut game_config_manager = match game_config_manager.lock() {
            Ok(game_config_manager) => {
                trace!("Locked game config manager mutex.");
                game_config_manager
            },
            Err(_) => panic!("Failed to lock game config manager mutex!"),
        };
        let game_info_manager = GAME_INFO_MANAGER.clone();
        let mut game_info_manager = match game_info_manager.lock() {
            Ok(game_info_manager) => {
                trace!("Locked game info manager mutex.");
                game_info_manager
            },
            Err(_) => panic!("Failed to lock game info manager mutex!"),
        };

        debug!("Locked game module manager mutexes.");

        info!("Finalizing game main module....");

        match game_state_manager.finalize() {
            Ok(_) => {
                trace!("Finalized game state module.");
            }
            Err(_) => {
                panic!("Failed to finalize game state module!");
            }
        };
        match game_config_manager.finalize() {
            Ok(_) => {
                trace!("Finalized game config module.");
            }
            Err(_) => {
                panic!("Failed to finalize game config module!");
            }
        };
        match game_info_manager.finalize() {
            Ok(_) => {
                trace!("Finalized game info module.");
            }
            Err(_) => {
                panic!("Failed to finalize game info module!");
            }
        };

        info!("Finalized game main module..");

        self.manager_state = ManagerState::Finalized;

        info!("Finalized game main module.");

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl GameManager {
    fn new() -> GameManager {
        GameManager {
            manager_state: ManagerState::Created,
            current_game: None,
        }
    }

    fn load_game(game_info: GameInfo) {
        info!("Loading game module...");

        let game_manager: Arc<Mutex<GameManager>> = GAME_MANAGER.clone();
        let mut game_manager = match game_manager.lock() {
            Ok(game_manager) => {
                trace!("Locked game manager mutex.");
                game_manager
            },
            Err(_) => panic!("Failed to lock game manager mutex!"),
        };

        match game_manager.current_game {
            None => {}
            Some(_) => {
                error!("Game already loaded!");
                return;
            }
        }

        let dir_path = format!(
            "mods/loo_cast_base_mod/data/games/{}/config",
            game_info.name
        );
        if !Path::new(&dir_path).exists() {
            trace!("Creating game config...");

            match std::fs::create_dir_all(&dir_path) {
                Ok(_) => {
                    trace!("Created config directory.")
                }
                Err(_) => {
                    error!("Failed to create config directory!");
                    return;
                }
            }

            let file_path = format!("{}/info.json", dir_path);
            match File::create(file_path) {
                Ok(_) => {
                    trace!("Created config/info.json.")
                }
                Err(_) => {
                    error!("Failed to create config/info.json!");
                    return;
                }
            };

            trace!("Created game config.");
        }

        debug!("Loading game config...");

        GameConfigManager::load_game_config();

        debug!("Loaded game config.");

        let dir_path = format!(
            "mods/loo_cast_base_mod/data/games/{}/state",
            game_info.name
        );
        if !Path::new(&dir_path).exists() {
            trace!("Creating game state...");

            match std::fs::create_dir_all(&dir_path) {
                Ok(_) => {
                    trace!("Created state directory.")
                }
                Err(_) => {
                    error!("Failed to create state directory!");
                    return;
                }
            };

            let file_path = format!("{}/info.json", dir_path);
            match File::create(file_path){
                Ok(_) => {
                    trace!("Created config/info.json for state.")
                }
                Err(_) => {
                    error!("Failed to create config/info.json for state!");
                    return;
                }
            };

            trace!("Created game state.");
        }

        debug!("Loading game state...");

        GameStateManager::load_game_state();

        debug!("Loaded game state.");

        game_manager.current_game = Some(game_info);

        info!("Loaded game module.");
    }

    fn unload_game() {
        info!("Unloading game module...");

        let game_manager: Arc<Mutex<GameManager>> = GAME_MANAGER.clone();
        let mut game_manager = match game_manager.lock() {
            Ok(camera_manager) => {
                trace!("Locked game manager mutex.");
                camera_manager
            },
            Err(_) => panic!("Failed to lock game manager mutex!"),
        };

        GameConfigManager::unload_game_config();

        match game_manager.current_game {
            None => {
                error!("Game already unloaded!");
                return;
            }
            Some(_) => {
                game_manager.current_game = None;
            }
        }

        info!("Unloaded game module.");
    }

    pub(in crate::system::game) fn handle_create_game(
        mut create_game_event_reader: EventReader<CreateGame>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
        for event in create_game_event_reader.iter() {
            let game_info_manager = GAME_INFO_MANAGER.clone();
            let mut game_info_manager = match game_info_manager.lock() {
                Ok(game_info_manager) => {
                    trace!("Locked game info manager mutex.");
                    game_info_manager
                },
                Err(_) => panic!("Failed to lock game info manager mutex!"),
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

            game_info_manager.register_game_info(game_info);

            app_state_next_state.set(AppState::GamesMenu);
        }
    }

    pub(in crate::system::game) fn handle_delete_game(
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

            let game_infos = match game_info_manager.get_game_infos_mut() {
                None => {
                    error!("Game infos not registered!");
                    continue;
                }
                Some(game_infos) => game_infos,
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
            game_infos.iter().enumerate()
            {
                if game_info.name == event.game_name {
                    index_to_remove = Some(index);
                    break;
                }
            }

            if let Some(index) = index_to_remove {
                game_infos.remove(index);
            }

            delete_game_ui_event_writer.send(DeleteGameUI {
                game_name: event.game_name.to_string(),
            });
        }
    }

    fn handle_load_game(
        mut load_game_event_reader: EventReader<LoadGame>,
        mut load_universe_event_writer: EventWriter<LoadGlobalUniverse>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        if let Some(confirm_loaded_game_event) = load_game_event_reader.iter().last() {
            let game_info: GameInfo = confirm_loaded_game_event.game.clone();

            GameManager::load_game(game_info);

            simulation_state_next_state.set(SimulationState::Paused);

            app_state_next_state.set(AppState::Game);

            load_universe_event_writer.send(LoadGlobalUniverse {});
        }
    }

    fn handle_unload_game(
        mut unload_game_event_reader: EventReader<UnloadGame>,
        mut app_exit_event_writer: EventWriter<AppExit>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        if let Some(unload_game_event) = unload_game_event_reader.iter().last() {
            GameManager::unload_game();

            simulation_state_next_state.set(SimulationState::Running);
            if unload_game_event.quit_mode == GameQuitMode::QuitToMainMenu {
                app_state_next_state.set(AppState::MainMenu);
            }

            if unload_game_event.quit_mode == GameQuitMode::QuitToDesktop {
                app_exit_event_writer.send(AppExit);
            }
        }
    }

    fn handle_toggle_simulation(
        keyboard_input: Res<Input<KeyCode>>,
        simulation_state: Res<State<SimulationState>>,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            if *simulation_state.get() == SimulationState::Running {
                simulation_state_next_state.set(SimulationState::Paused);
                info!("Simulation Paused.");
            }
            if *simulation_state.get() == SimulationState::Paused {
                simulation_state_next_state.set(SimulationState::Running);
                info!("Simulation Running.");
            }
        }
    }
}

// Module Functions
