use bevy::prelude::*;

#[derive(Event)]
pub struct CreateSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeleteSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct LoadSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct ConfirmCreatedSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct ConfirmDeletedSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct ConfirmLoadedSaveGame {
    pub save_game_name: String,
}
