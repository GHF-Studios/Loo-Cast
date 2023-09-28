// External imports
use bevy::prelude::*;

// Resources
#[derive(Resource)]
pub struct GameStateManager;

// Implementations
impl GameStateManager {
    pub fn initialize(commands: &mut Commands) {
        commands.insert_resource(GameStateManager {})
    }

    pub fn terminate(commands: &mut Commands) {
        commands.remove_resource::<GameStateManager>();
    }
}
