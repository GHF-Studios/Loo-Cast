mod create_save_game_menu;
pub mod events;
mod input_field;
mod main_menu;
mod pause_menu;
pub mod resources;
mod save_games_menu;
pub mod styles;
mod systems;

use create_save_game_menu::CreateSaveGameMenuPlugin;
use input_field::InputFieldPlugin;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;
use save_games_menu::SaveGamesMenuPlugin;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize Events
            .add_event::<GainedFocus>()
            .add_event::<LostFocus>()
            // Initialize Resources
            .init_resource::<FocusManager>()
            // Plugins
            .add_plugins((
                CreateSaveGameMenuPlugin,
                MainMenuPlugin,
                PauseMenuPlugin,
                SaveGamesMenuPlugin,
                InputFieldPlugin,
            ))
            // Update Systems
            .add_systems(Update, handle_gained_focus_event);
    }
}
