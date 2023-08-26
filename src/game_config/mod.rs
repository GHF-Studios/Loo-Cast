mod systems;

pub mod events;
pub mod resources;

use events::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGameConfig>()
            .add_event::<ConfirmLoadedGameConfig>()
            // Update Systems
            .add_systems(Update, handle_load_game_config.run_if(in_state(AppState::Game)));
    }
}
