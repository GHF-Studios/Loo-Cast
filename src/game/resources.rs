use bevy::prelude::*;

use crate::save_game::structs::SaveGameInfo;

#[derive(Resource)]
pub struct GameManager {
    pub current_save_game: SaveGameInfo,
}
