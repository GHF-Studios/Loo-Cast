pub mod events;
pub mod resources;
mod systems;
mod config;
mod state;

use events::*;
use systems::*;
use config::*;
use state::*;

use crate::AppState;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((ConfigPlugin, StatePlugin))
            // Events
            .add_event::<LoadGame>()
            .add_event::<LoadedGame>()
            .add_event::<UnloadGame>()
            // States
            .add_state::<SimulationState>()
            .add_state::<LoadState>()
            // Update Systems
            .add_systems(
                Update,
                handle_load_game
                .run_if(in_state(AppState::SaveGamesMenu)),
            )
            .add_systems(
                Update,
                (toggle_simulation, handle_unload_game)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LoadState {
    #[default]
    LoadedSaveGame,
    LoadedGameConfig,
    LoadedGameState,
    LoadedUniverse,
    FullyLoaded,
}
