use crate::save_game::events::*;
use crate::save_game::resources::*;
use crate::save_game::structs::*;
use crate::AppState;

use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn handle_create_save_game(
    mut create_save_game_event_reader: EventReader<CreateSaveGame>,
    mut confirm_created_save_game_event_writer: EventWriter<ConfirmCreatedSaveGame>,
    mut save_game_manager: ResMut<SaveGameManager>,
) {
    for event in create_save_game_event_reader.iter() {
        let save_game_info: SaveGameInfo = SaveGameInfo {
            name: event.save_game_name.to_string(),
        };

        let serialized_save_game_info: String = serde_json::to_string(&save_game_info).unwrap();

        let string_path = format!("data/saves/{}.json", event.save_game_name);
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
        confirm_created_save_game_event_writer.send(ConfirmCreatedSaveGame {
            save_game_name: event.save_game_name.to_string(),
        });
    }
}

pub fn handle_deleted_save_game(
    mut delete_save_game_event_reader: EventReader<DeleteSaveGame>,
    mut confirm_deleted_save_game_event_writer: EventWriter<ConfirmDeletedSaveGame>,
    mut save_game_manager: ResMut<SaveGameManager>,
) {
    for event in delete_save_game_event_reader.iter() {
        let string_path = format!("data/saves/{}.json", event.save_game_name);
        let path = Path::new(&string_path);
        let display = path.display();

        match std::fs::remove_file(&path) {
            Err(why) => panic!("Couldn't delete {}: {}", display, why),
            Ok(_) => println!("successfully deleted {}", display),
        }

        // get the game info from the save game manager, remove it, delete it, and send the confirm, event
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

        confirm_deleted_save_game_event_writer.send(ConfirmDeletedSaveGame {
            save_game_name: event.save_game_name.to_string(),
        });
    }
}

pub fn handle_loaded_save_game(
    mut load_save_game_event_reader: EventReader<LoadSaveGame>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Some(_) = load_save_game_event_reader.iter().next() {
        app_state_next_state.set(AppState::Game);
    }
}
