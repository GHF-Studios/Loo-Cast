mod systems;

pub mod background;
pub mod camera;
pub mod game;
pub mod math;
pub mod player;
pub mod ui;

use systems::*;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use player::PlayerPlugin;
use ui::UIPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // States
        .add_state::<AppState>()
        // Plugins
        .add_plugin(BackgroundPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UIPlugin)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        // Run
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
