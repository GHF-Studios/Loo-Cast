mod components;
mod resources;
mod systems;

use systems::*;

use crate::game::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(insert_background_manager.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_background.in_schedule(OnEnter(AppState::Game)))
            // OnUpdate Systems
            .add_system(
                move_background
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // OnExit Systems
            .add_system(despawn_background.in_schedule(OnExit(AppState::Game)))
            .add_system(remove_background_manager.in_schedule(OnExit(AppState::Game)));
    }
}
