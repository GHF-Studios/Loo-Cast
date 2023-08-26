mod systems;

pub mod events;
pub mod resources;

use events::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGameState>()
            .add_event::<ConfirmLoadedGameState>()
            // Update Systems
            .add_systems(Update, handle_load_game_state.run_if(in_state(AppState::Game)));
    }
}
