mod systems;

pub mod background;
pub mod camera;
pub mod game;
pub mod math;
pub mod player;
mod save_game;
pub mod ui;
pub mod universe;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use player::PlayerPlugin;
use save_game::SaveGamePlugin;
use ui::UIPlugin;
use universe::UniversePlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // States
        .add_state::<AppState>()
        // Plugins
        .add_plugins((
            BackgroundPlugin,
            CameraPlugin,
            GamePlugin,
            PlayerPlugin,
            SaveGamePlugin,
            UIPlugin,
            UniversePlugin,
        ))
        // Update Systems
        // Run
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    SaveGamesMenu,
    CreateSaveGameMenu,
    Game,
}
