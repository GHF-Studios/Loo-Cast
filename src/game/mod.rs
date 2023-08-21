mod systems;

use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
