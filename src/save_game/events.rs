use bevy::prelude::*;

#[derive(Event)]
pub struct CreateSaveGameEvent {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct DeleteSaveGameEvent {
    pub save_game_name: String,
}

#[derive(Event)]
pub struct LoadSaveGameEvent {
    pub save_game_name: String,
}
