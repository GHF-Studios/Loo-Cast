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
        .add_plugins((BackgroundPlugin, CameraPlugin, GamePlugin, PlayerPlugin, UIPlugin))
        // Update Systems
        .add_systems(Update, (transition_to_game_state, transition_to_main_menu_state, exit_game))
        // Run
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}
