// Modules
pub mod main_menu;
pub mod game_creation_menu;
pub mod games_menu;

// Local imports
use main_menu::MainMenuPlugin;
use game_creation_menu::GameCreationMenuPlugin;
use games_menu::GamesMenuPlugin;

// Internal imports

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Traits

// Enums

// Structs
pub struct MenuPlugin;

#[derive(Resource, Default)]
pub struct MenuManager {
}

// Implementations
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((
                MainMenuPlugin,
                GameCreationMenuPlugin,
                GamesMenuPlugin,
            ));
    }
}

impl MenuManager {
}

// Module Functions
