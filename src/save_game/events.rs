use bevy::prelude::*;

use super::enums::GameQuitMode;
use super::structs::SaveGameInfo;

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
pub struct UnloadSaveGame {
    pub quit_mode: GameQuitMode,
}

#[derive(Event)]
pub struct ConfirmCreatedSaveGame {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct ConfirmDeletedSaveGame {
    pub save_game: String,
}

#[derive(Event)]
pub struct ConfirmLoadedSaveGame {
    pub save_game: SaveGameInfo,
}

#[derive(Event)]
pub struct ConfirmUnloadedSaveGame {
    pub quit_mode: GameQuitMode,
}
