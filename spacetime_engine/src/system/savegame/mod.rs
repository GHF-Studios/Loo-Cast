// Modules

// Local imports

// Internal imports
use crate::system::ui::savegames_menu::*;
use crate::system::AppState;

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
pub struct SavegamePlugin;

#[derive(Serialize, Deserialize, Clone)]
pub struct SavegameInfo {
    pub name: String,
}

#[derive(Event)]
pub struct CreateSavegame {
    pub savegame_name: String,
}

#[derive(Event)]
pub struct DeleteSavegame {
    pub savegame_name: String,
}

#[derive(Resource, Default)]
pub struct SavegameManager {
    pub registered_savegames: Vec<SavegameInfo>,
}

// Implementations
impl Plugin for SavegamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<CreateSavegame>()
            .add_event::<DeleteSavegame>()
            // Startup Systems
            .add_systems(Startup, SavegameManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (SavegameManager::handle_delete_savegame)
                    .run_if(in_state(AppState::SavegamesMenu)),
            )
            .add_systems(
                Update,
                (SavegameManager::handle_create_savegame)
                    .run_if(in_state(AppState::CreateSavegameMenu)),
            );
    }
}

impl SavegameManager {
    fn initialize(mut commands: Commands) {
        let mut savegame_infos: Vec<SavegameInfo> = Vec::new();

        if let Ok(paths) = std::fs::read_dir("loo_cast_base_mod/data/saves") {
            for path in paths {
                let path = path.unwrap().path();
                if path.is_dir() {
                    let info_path = path.join("info.json");
                    let display = info_path.display();

                    let mut file = match File::open(&info_path) {
                        Err(why) => panic!("Couldn't open {}: ({})", display, why),
                        Ok(file) => file,
                    };

                    let mut serialized_savegame_info = String::new();
                    match file.read_to_string(&mut serialized_savegame_info) {
                        Err(why) => panic!("Couldn't read {}: ({})", display, why),
                        Ok(_) => println!("Successfully read {}", display),
                    }

                    savegame_infos.push(serde_json::from_str(&serialized_savegame_info).unwrap());
                }
            }
        }

        commands.insert_resource(SavegameManager {
            registered_savegames: savegame_infos,
        });
    }

    fn handle_create_savegame(
        mut create_savegame_event_reader: EventReader<CreateSavegame>,
        mut savegame_manager: ResMut<SavegameManager>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
        for event in create_savegame_event_reader.iter() {
            let savegame_info: SavegameInfo = SavegameInfo {
                name: event.savegame_name.to_string(),
            };

            let serialized_savegame_info: String = serde_json::to_string(&savegame_info).unwrap();

            let dir_path = format!("mods/loo_cast_base_mod/data/saves/{}", event.savegame_name);
            if !Path::new(&dir_path).exists() {
                std::fs::create_dir_all(&dir_path).expect("Failed to create save game directory");
            }

            let string_path = format!("{}/info.json", dir_path);
            let path = Path::new(&string_path);
            let display = path.display();

            let mut file = match File::create(path) {
                Err(why) => panic!("Couldn't create {}: ({})", display, why),
                Ok(file) => file,
            };

            match file.write_all(serialized_savegame_info.as_bytes()) {
                Err(why) => panic!("Couldn't write to {}: ({})", display, why),
                Ok(_) => println!("Successfully wrote to {}", display),
            }

            savegame_manager.registered_savegames.push(savegame_info);
            app_state_next_state.set(AppState::SavegamesMenu);
        }
    }

    fn handle_delete_savegame(
        mut delete_savegame_event_reader: EventReader<DeleteSavegame>,
        mut delete_savegame_ui_event_writer: EventWriter<DeleteSavegameUI>,
        mut savegame_manager: ResMut<SavegameManager>,
    ) {
        for event in delete_savegame_event_reader.iter() {
            let dir_path = format!("mods/loo_cast_base_mod/data/saves/{}", event.savegame_name);
            let string_path = format!("{}/info.json", dir_path);
            let path = Path::new(&string_path);
            let display = path.display();

            match std::fs::remove_file(path) {
                Err(why) => panic!("Couldn't delete {}: ({})", display, why),
                Ok(_) => println!("Successfully deleted {}", display),
            }

            std::fs::remove_dir_all(&dir_path).expect("Failed to remove save game directory");

            let mut index_to_remove: Option<usize> = None;
            for (index, savegame_info) in
                savegame_manager.registered_savegames.iter().enumerate()
            {
                if savegame_info.name == event.savegame_name {
                    index_to_remove = Some(index);
                    break;
                }
            }

            if let Some(index) = index_to_remove {
                savegame_manager.registered_savegames.remove(index);
            }

            delete_savegame_ui_event_writer.send(DeleteSavegameUI {
                savegame_name: event.savegame_name.to_string(),
            });
        }
    }

    pub fn get_savegame_info(&self, savegame_name: String) -> Option<&SavegameInfo> {
        self.registered_savegames
            .iter()
            .find(|&savegame| savegame.name == *savegame_name)
    }
}

// Module Functions
