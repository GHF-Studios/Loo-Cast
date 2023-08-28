use crate::save_game::events::*;
use crate::save_game::resources::*;
use crate::save_game::structs::*;
use crate::ui::save_games_menu::events::DeleteSaveGameUI;
use crate::AppState;

use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn handle_create_save_game(
    mut create_save_game_event_reader: EventReader<CreateSaveGame>,
    mut save_game_manager: ResMut<SaveGameManager>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in create_save_game_event_reader.iter() {
        let save_game_info: SaveGameInfo = SaveGameInfo {
            name: event.save_game_name.to_string(),
        };

        let serialized_save_game_info: String = serde_json::to_string(&save_game_info).unwrap();

        let dir_path = format!("data/saves/{}", event.save_game_name);
        if !Path::new(&dir_path).exists() {
            std::fs::create_dir_all(&dir_path).expect("Failed to create save game directory");
        }

        let string_path = format!("{}/info.json", dir_path);
        let path = Path::new(&string_path);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(serialized_save_game_info.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        save_game_manager.registered_save_games.push(save_game_info);
        app_state_next_state.set(AppState::SaveGamesMenu);
    }
}

pub fn handle_delete_save_game(
    mut delete_save_game_event_reader: EventReader<DeleteSaveGame>,
    mut delete_save_game_ui_event_writer: EventWriter<DeleteSaveGameUI>,
    mut save_game_manager: ResMut<SaveGameManager>,
) {
    for event in delete_save_game_event_reader.iter() {
        let dir_path = format!("data/saves/{}", event.save_game_name);
        let string_path = format!("{}/info.json", dir_path);
        let path = Path::new(&string_path);
        let display = path.display();

        match std::fs::remove_file(&path) {
            Err(why) => panic!("Couldn't delete {}: {}", display, why),
            Ok(_) => println!("successfully deleted {}", display),
        }

        std::fs::remove_dir_all(&dir_path).expect("Failed to remove save game directory");

        let mut index_to_remove: Option<usize> = None;
        for (index, save_game_info) in save_game_manager.registered_save_games.iter().enumerate() {
            if save_game_info.name == event.save_game_name {
                index_to_remove = Some(index);
                break;
            }
        }

        if let Some(index) = index_to_remove {
            save_game_manager.registered_save_games.remove(index);
        }

        delete_save_game_ui_event_writer.send(
            crate::ui::save_games_menu::events::DeleteSaveGameUI {
                save_game_name: event.save_game_name.to_string(),
            },
        );
    }
}

pub fn init(mut commands: Commands) {
    let paths = std::fs::read_dir("data/saves").unwrap();
    let mut save_game_infos: Vec<SaveGameInfo> = Vec::new();

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

    commands.insert_resource(SaveGameManager {
        registered_save_games: save_game_infos,
    });
}
