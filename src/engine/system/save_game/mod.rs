// Modules

// Local imports

// Internal imports
use crate::engine::system::ui::save_games_menu::*;
use crate::engine::system::AppState;

// External imports
use bevy::prelude::*;
use serde::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Static variables

// Constant variables

// Types

// Enums
#[derive(Debug, Clone, PartialEq)]
pub enum GameQuitMode {
    QuitToMainMenu,
    QuitToDesktop,
}

// Structs
pub struct SaveGamePlugin;

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveGameInfo {
    pub name: String,
}

#[derive(Event)]
pub struct CreateSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeleteSaveGame {
    pub save_game_name: String,
}

#[derive(Resource)]
#[derive(Default)]
pub struct SaveGameManager {
    pub registered_save_games: Vec<SaveGameInfo>,
}

// Implementations
impl Plugin for SaveGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<CreateSaveGame>()
            .add_event::<DeleteSaveGame>()
            // Startup Systems
            .add_systems(Startup, SaveGameManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (SaveGameManager::handle_delete_save_game)
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            .add_systems(
                Update,
                (SaveGameManager::handle_create_save_game)
                    .run_if(in_state(AppState::CreateSaveGameMenu)),
            );
    }
}

impl SaveGameManager {
    fn initialize(mut commands: Commands) {
        let mut save_game_infos: Vec<SaveGameInfo> = Vec::new();

        if let Ok(paths) = std::fs::read_dir("assets/data/saves") {
            for path in paths {
                let path = path.unwrap().path();
                if path.is_dir() {
                    let info_path = path.join("info.json");
                    let display = info_path.display();

                    let mut file = match File::open(&info_path) {
                        Err(why) => panic!("Couldn't open {}: {}", display, why),
                        Ok(file) => file,
                    };

                    let mut serialized_save_game_info = String::new();
                    match file.read_to_string(&mut serialized_save_game_info) {
                        Err(why) => panic!("Couldn't read {}: {}", display, why),
                        Ok(_) => println!("Successfully read {}", display),
                    }

                    save_game_infos.push(serde_json::from_str(&serialized_save_game_info).unwrap());
                }
            }
        }

        commands.insert_resource(SaveGameManager {
            registered_save_games: save_game_infos,
        });
    }

    fn handle_create_save_game(
        mut create_save_game_event_reader: EventReader<CreateSaveGame>,
        mut save_game_manager: ResMut<SaveGameManager>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
        for event in create_save_game_event_reader.iter() {
            let save_game_info: SaveGameInfo = SaveGameInfo {
                name: event.save_game_name.to_string(),
            };

            let serialized_save_game_info: String = serde_json::to_string(&save_game_info).unwrap();

            let dir_path = format!("assets/data/saves/{}", event.save_game_name);
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

            match file.write_all(serialized_save_game_info.as_bytes()) {
                Err(why) => panic!("Couldn't write to {}: {}", display, why),
                Ok(_) => println!("Successfully wrote to {}", display),
            }

            save_game_manager.registered_save_games.push(save_game_info);
            app_state_next_state.set(AppState::SaveGamesMenu);
        }
    }

    fn handle_delete_save_game(
        mut delete_save_game_event_reader: EventReader<DeleteSaveGame>,
        mut delete_save_game_ui_event_writer: EventWriter<DeleteSaveGameUI>,
        mut save_game_manager: ResMut<SaveGameManager>,
    ) {
        for event in delete_save_game_event_reader.iter() {
            let dir_path = format!("assets/data/saves/{}", event.save_game_name);
            let string_path = format!("{}/info.json", dir_path);
            let path = Path::new(&string_path);
            let display = path.display();

            match std::fs::remove_file(path) {
                Err(why) => panic!("Couldn't delete {}: {}", display, why),
                Ok(_) => println!("Successfully deleted {}", display),
            }

            std::fs::remove_dir_all(&dir_path).expect("Failed to remove save game directory");

            let mut index_to_remove: Option<usize> = None;
            for (index, save_game_info) in
                save_game_manager.registered_save_games.iter().enumerate()
            {
                if save_game_info.name == event.save_game_name {
                    index_to_remove = Some(index);
                    break;
                }
            }

            if let Some(index) = index_to_remove {
                save_game_manager.registered_save_games.remove(index);
            }

            delete_save_game_ui_event_writer.send(DeleteSaveGameUI {
                save_game_name: event.save_game_name.to_string(),
            });
        }
    }

    pub fn get_save_game_info(&self, save_game_name: String) -> Option<&SaveGameInfo> {
        self.registered_save_games
            .iter()
            .find(|&save_game| save_game.name == *save_game_name)
    }
}



// Module Functions
