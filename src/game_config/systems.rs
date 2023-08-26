use crate::game::resources::*;
use crate::game_config::events::*;
use crate::save_game::structs::*;

use super::resources::*;

use bevy::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn handle_load_game_config(
    mut commands: Commands,
    mut load_game_config_event_reader: EventReader<LoadGameConfig>,
    mut confirm_loaded_game_config_event_writer: EventWriter<ConfirmLoadedGameConfig>,
    game_manager: Res<GameManager>,
) {
    if let Some(_) = load_game_config_event_reader.iter().last() {
        let save_game_info: SaveGameInfo = game_manager.current_save_game.clone();

        let dir_path = format!("data/saves/{}/config", save_game_info.name);
        if !Path::new(&dir_path).exists() {
            std::fs::create_dir_all(&dir_path).expect("Failed to create config directory");

            let file_path = format!("{}/info.json", dir_path);
            File::create(&file_path).expect("Failed to create info.json for config");
        }

        commands.insert_resource(GameConfigManager{});

        confirm_loaded_game_config_event_writer.send(ConfirmLoadedGameConfig {});
    }
}
