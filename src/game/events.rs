use crate::save_game::{structs::SaveGameInfo, enums::GameQuitMode};


use bevy::prelude::*;

#[derive(Event)]
pub struct LoadGame {
    pub save_game: SaveGameInfo,
}

#[derive(Event)]
pub struct LoadedGame;

#[derive(Event)]
pub struct UnloadGame {
    pub quit_mode: GameQuitMode,
}
