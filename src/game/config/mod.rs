// Modules


// Local imports


// Internal imports


// External imports
use bevy::prelude::*;

// Static variables


// Constant variables


// Types


// Enums


// Structs
#[derive(Resource)]
pub struct GameConfigManager;

// Implementations
impl GameConfigManager {
    pub fn initialize(commands: &mut Commands) {
        commands.insert_resource(GameConfigManager {})
    }

    pub fn terminate(commands: &mut Commands) {
        commands.remove_resource::<GameConfigManager>();
    }
}

// Module Functions
