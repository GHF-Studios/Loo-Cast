use super::structs::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct SaveGameManager {
    pub registered_save_games: Vec<SaveGameInfo>,
}

impl Default for SaveGameManager {
    fn default() -> SaveGameManager {
        SaveGameManager {
            registered_save_games: Vec::new(),
        }
    }
}
