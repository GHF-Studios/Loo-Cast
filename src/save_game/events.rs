use bevy::prelude::*;

#[derive(Event)]
pub struct CreateSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeleteSaveGame {
    pub save_game_name: String,
}
