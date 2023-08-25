use super::structs::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct SaveGameManager {
    pub registered_save_games: Vec<SaveGameInfo>,
}

impl SaveGameManager {
    pub fn get_save_game_info(&self, save_game_name: &String) -> Option<&SaveGameInfo> {
        self.registered_save_games.iter().find(|&save_game| save_game.name == *save_game_name)
    }
}

impl Default for SaveGameManager {
    fn default() -> SaveGameManager {
        SaveGameManager {
            registered_save_games: Vec::new(),
        }
    }
}