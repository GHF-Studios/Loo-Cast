mod systems;

pub mod camera;
pub mod game;
pub mod player;
pub mod ui;

use systems::*;

use camera::CameraPlugin;
use game::GamePlugin;
use player::PlayerPlugin;
use ui::UIPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // States
        .add_state::<AppState>()
        // Plugins
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
