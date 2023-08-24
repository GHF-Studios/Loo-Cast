mod create_save_game_menu;
mod main_menu;
mod pause_menu;
mod save_games_menu;
pub mod styles;
mod input_field;

use create_save_game_menu::CreateSaveGameMenuPlugin;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;
use save_games_menu::SaveGamesMenuPlugin;
use input_field::InputFieldPlugin;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((
                CreateSaveGameMenuPlugin,
                MainMenuPlugin,
                PauseMenuPlugin,
                SaveGamesMenuPlugin,
                InputFieldPlugin
            ));
    }
}
