// Modules
pub mod config;
pub mod state;

// Local imports
use config::*;
use state::*;

// Internal imports
use crate::system::savegame::*;
//use crate::system::universe::*;
// TODO: Reimport universe commands
use crate::system::AppState;

// External imports
use bevy::app::AppExit;
use bevy::prelude::*;
use std::fs::File;
use std::path::Path;

// Static variables

// Constant variables

// Types

// Enums
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LoadState {
    #[default]
    LoadedSavegame,
    LoadedGameConfig,
    LoadedGameState,
    LoadedUniverse,
    FullyLoaded,
}

// Structs
pub struct GamePlugin;

#[derive(Event)]
pub struct LoadGame {
    pub savegame: SavegameInfo,
}

#[derive(Event)]
pub struct UnloadGame {
    pub quit_mode: GameQuitMode,
}

#[derive(Resource)]
pub struct GameManager {
    pub current_savegame: SavegameInfo,
}

// Implementations
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadGame>()
            .add_event::<UnloadGame>()
            // States
            .add_state::<SimulationState>()
            .add_state::<LoadState>()
            // Update Systems
            .add_systems(
                Update,
                GameManager::handle_load_game.run_if(in_state(AppState::SavegamesMenu)),
            )
            .add_systems(
                Update,
                (
                    GameManager::handle_toggle_simulation,
                    GameManager::handle_unload_game,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

impl GameManager {
    fn startup(commands: &mut Commands, savegame_info: SavegameInfo) {
        commands.insert_resource(GameManager {
            current_savegame: savegame_info,
        })
    }

    fn shutdown(commands: &mut Commands) {
        commands.remove_resource::<GameManager>();
    }

    fn handle_load_game(
        mut commands: Commands,
        mut load_game_event_reader: EventReader<LoadGame>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        if let Some(confirm_loaded_savegame_event) = load_game_event_reader.iter().last() {
            let savegame_info: SavegameInfo = confirm_loaded_savegame_event.savegame.clone();

            // Load Game Manager
            GameManager::startup(&mut commands, savegame_info.clone());

            // Load Game Config
            let dir_path = format!(
                "mods/loo_cast_base_mod/data/saves/{}/config",
                savegame_info.name
            );
            if !Path::new(&dir_path).exists() {
                std::fs::create_dir_all(&dir_path).expect("Failed to create config directory");

                let file_path = format!("{}/info.json", dir_path);
                File::create(file_path).expect("Failed to create info.json for config");
            }
            GameConfigManager::initialize(&mut commands);

            // Load Game State
            let dir_path = format!(
                "mods/loo_cast_base_mod/data/saves/{}/state",
                savegame_info.name
            );
            if !Path::new(&dir_path).exists() {
                std::fs::create_dir_all(&dir_path).expect("Failed to create state directory");

                let file_path = format!("{}/info.json", dir_path);
                File::create(file_path).expect("Failed to create info.json for state");
            }
            GameStateManager::initialize(&mut commands);

            // Finalize Loading
            simulation_state_next_state.set(SimulationState::Paused);
            app_state_next_state.set(AppState::Game);
        }
    }

    fn handle_unload_game(
        mut commands: Commands,
        mut unload_game_event_reader: EventReader<UnloadGame>,
        mut app_exit_event_writer: EventWriter<AppExit>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        if let Some(unload_savegame_event) = unload_game_event_reader.iter().last() {
            // Unload Game State
            GameStateManager::terminate(&mut commands);

            // Unload Game Config
            GameConfigManager::terminate(&mut commands);

            // Unload Game Manager
            GameManager::shutdown(&mut commands);

            // Finalize Unloading
            simulation_state_next_state.set(SimulationState::Running);
            if unload_savegame_event.quit_mode == GameQuitMode::QuitToMainMenu {
                app_state_next_state.set(AppState::MainMenu);
            }
            if unload_savegame_event.quit_mode == GameQuitMode::QuitToDesktop {
                app_exit_event_writer.send(AppExit);
            }
        }
    }

    pub fn handle_toggle_simulation(
        keyboard_input: Res<Input<KeyCode>>,
        simulation_state: Res<State<SimulationState>>,
        mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            if *simulation_state.get() == SimulationState::Running {
                simulation_state_next_state.set(SimulationState::Paused);
                println!("Simulation Paused.");
            }
            if *simulation_state.get() == SimulationState::Paused {
                simulation_state_next_state.set(SimulationState::Running);
                println!("Simulation Running.");
            }
        }
    }
}

// Module Functions
