mod components;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::SimulationState;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter Systems
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(player_movement
                .in_set(OnUpdate(AppState::Game))
                .run_if(in_state(SimulationState::Running)))
            // On Exit Systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
