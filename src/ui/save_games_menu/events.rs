use bevy::prelude::*;

#[derive(Event)]
pub struct LoadSaveGameInstance {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeleteSaveGameUI {
    pub save_game_name: String,
}
