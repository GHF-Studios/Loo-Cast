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
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // Update Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
