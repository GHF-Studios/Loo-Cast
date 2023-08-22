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
            // On Enter Systems
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(
                player_movement
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit Systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
