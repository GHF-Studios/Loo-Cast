mod systems;

pub mod components;

use systems::*;

use crate::game::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Update Systems
            .add_systems(Update, player_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
