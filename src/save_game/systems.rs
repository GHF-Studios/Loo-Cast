use crate::save_game::events::*;
use crate::save_game::resources::*;
use crate::save_game::structs::*;

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

pub fn handle_delete_save_game(
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
            save_game: event.save_game_name.to_string(),
        });
    }
}

pub fn handle_load_save_game(
    mut load_save_game_event_reader: EventReader<LoadSaveGame>,
    mut confirm_loaded_save_game_event_writer: EventWriter<ConfirmLoadedSaveGame>,
    save_game_manager: Res<SaveGameManager>,
) {
    if let Some(loaded_save_game_event) = load_save_game_event_reader.iter().last() {
        if let Some(save_game) = save_game_manager.get_save_game_info(&loaded_save_game_event.save_game_name) {
            confirm_loaded_save_game_event_writer.send(ConfirmLoadedSaveGame {
                save_game: save_game.clone(),
            });
        }
    }
}

pub fn handle_unload_save_game(
    mut unload_save_game_event_reader: EventReader<UnloadSaveGame>,
    mut confirm_unloaded_save_game_event_writer: EventWriter<ConfirmUnloadedSaveGame>,
) {
    if let Some(unloaded_save_game_event) = unload_save_game_event_reader.iter().last() {
        confirm_unloaded_save_game_event_writer.send(ConfirmUnloadedSaveGame {
            quit_mode: unloaded_save_game_event.quit_mode.clone(),
        });
    }
}

pub fn init_save_game_manager(
    mut commands: Commands,
) {
    let paths = std::fs::read_dir("data/saves").unwrap();
    let mut save_game_infos: Vec<SaveGameInfo> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let display = path.display();

        let mut file = match File::open(&path) {
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

    commands.insert_resource(SaveGameManager {
        registered_save_games: save_game_infos,
    });
}