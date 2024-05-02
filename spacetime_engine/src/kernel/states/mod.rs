use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ApplicationState {
    #[default]
    Void,
    Loading,
    Menu,
    Game,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState {
    #[default]
    MainMenu,
    SettingsMenu,
    CreditsMenu,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Paused,
    Playing,
    GameOver,
}