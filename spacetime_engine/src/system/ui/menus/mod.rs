// Modules
pub mod game_creation_menu;
pub mod games_menu;
pub mod main_menu;

// Local imports
use game_creation_menu::GameCreationMenuPlugin;
use games_menu::GamesMenuPlugin;
use main_menu::MainMenuPlugin;

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
pub struct MenuManager {}

// Implementations
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((MainMenuPlugin, GameCreationMenuPlugin, GamesMenuPlugin));
    }
}

impl MenuManager {}

// Module Functions
