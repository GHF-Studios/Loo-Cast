use bevy::prelude::*;

#[derive(Event)]
pub struct CreatedSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeletedSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct LoadedSaveGame {
    pub save_game_name: String,
}
