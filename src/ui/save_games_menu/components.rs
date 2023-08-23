use bevy::prelude::Component;

#[derive(Component)]
pub struct SaveGamesMenu {}

#[derive(Component)]
pub struct SaveGamesContainer {}

#[derive(Component)]
pub struct SaveGame {
    pub save_game_name: String,
}

#[derive(Component)]
pub struct CreateSaveGameButton {}

#[derive(Component)]
pub struct BackToMainMenuButton {}

#[derive(Component)]
pub struct LoadSaveGameButton {}

#[derive(Component)]
pub struct DeleteSaveGameButton {}
