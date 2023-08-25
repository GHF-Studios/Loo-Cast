mod systems;

pub mod background;
pub mod camera;
pub mod game;
mod game_configuration;
pub mod math;
pub mod player;
mod save_game;
pub mod ui;
pub mod universe;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use game_configuration::GameConfigurationPlugin;
use player::PlayerPlugin;
use save_game::SaveGamePlugin;
use ui::UIPlugin;
use universe::UniversePlugin;

use bevy::{prelude::*, app::PluginGroupBuilder};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    SaveGamesMenu,
    CreateSaveGameMenu,
    Game,
}

pub struct LooCastBasePlugins;

impl PluginGroup for LooCastBasePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(BackgroundPlugin)
            .add(CameraPlugin)
            .add(GamePlugin)
            .add(GameConfigurationPlugin)
            .add(PlayerPlugin)
            .add(SaveGamePlugin)
            .add(UIPlugin)
            .add(UniversePlugin);
        
        group
    }
}

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // States
        .add_state::<AppState>()
        // Loo Cast Base Plugins
        .add_plugins(LooCastBasePlugins)
        // Run
        .run();
}
