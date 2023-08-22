mod components;
mod resources;
mod systems;

use systems::*;

use crate::game::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct BackgroundPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(
                OnEnter(AppState::Game),
                (insert_background_manager, spawn_background),
            )
            // Update Systems
            .add_systems(
                Update,
                move_background
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(
                OnExit(AppState::Game),
                (despawn_background, remove_background_manager),
            );
    }
}
