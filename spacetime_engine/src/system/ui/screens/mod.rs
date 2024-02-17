// Modules
pub mod game_creation_screen;
pub mod games_screen;
pub mod main_screen;

// Local imports
use game_creation_screen::GameCreationScreenPlugin;
use games_screen::GamesScreenPlugin;
use main_screen::MainScreenPlugin;

// Internal imports

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Traits

// Enums

// Structs
pub struct ScreenPlugin;

#[derive(Resource, Default)]
pub struct ScreenManager {}

// Implementations
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((MainScreenPlugin, GameCreationScreenPlugin, GamesScreenPlugin));
    }
}

impl ScreenManager {}

// Module Functions
