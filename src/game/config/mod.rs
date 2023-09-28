// External imports
use bevy::prelude::*;

// Resources
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
