use crate::game::config::resources::*;
use crate::game::state::resources::*;
use crate::game::SimulationState;
use crate::save_game::enums::GameQuitMode;
use crate::save_game::structs::*;
use crate::universe::events::*;
use crate::AppState;

use super::events::*;
use super::resources::GameManager;

use bevy::app::AppExit;
use bevy::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn toggle_simulation(
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

pub fn handle_load_game(
    mut commands: Commands,
    mut load_game_event_reader: EventReader<LoadGame>,
    mut load_universe_event_writer: EventWriter<LoadUniverse>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Some(confirm_loaded_save_game_event) = load_game_event_reader.iter().last() {
        let save_game_info: SaveGameInfo = confirm_loaded_save_game_event.save_game.clone();

        // Load Game Manager
        commands.insert_resource(GameManager {
            current_save_game: save_game_info.clone(),
        });

        // Load Game Config
        let dir_path = format!("assets/data/saves/{}/config", save_game_info.name);
        if !Path::new(&dir_path).exists() {
            std::fs::create_dir_all(&dir_path).expect("Failed to create config directory");

            let file_path = format!("{}/info.json", dir_path);
            File::create(&file_path).expect("Failed to create info.json for config");
        }

        commands.insert_resource(GameConfigManager {});

        // Load Game State
        let dir_path = format!("assets/data/saves/{}/state", save_game_info.name);
        if !Path::new(&dir_path).exists() {
            std::fs::create_dir_all(&dir_path).expect("Failed to create state directory");

            let file_path = format!("{}/info.json", dir_path);
            File::create(&file_path).expect("Failed to create info.json for state");
        }
        commands.insert_resource(GameStateManager {});

        // Finalize Loading
        simulation_state_next_state.set(SimulationState::Paused);
        app_state_next_state.set(AppState::Game);
        load_universe_event_writer.send(LoadUniverse {});
    }
}

pub fn handle_unload_game(
    mut commands: Commands,
    mut unload_game_event_reader: EventReader<UnloadGame>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Some(unload_save_game_event) = unload_game_event_reader.iter().last() {
        // Unload Game State
        commands.remove_resource::<GameStateManager>();

        // Unload Game Config
        commands.remove_resource::<GameConfigManager>();

        // Unload Game Manager
        commands.remove_resource::<GameManager>();

        // Finalize Unloading
        simulation_state_next_state.set(SimulationState::Running);
        if unload_save_game_event.quit_mode == GameQuitMode::QuitToMainMenu {
            app_state_next_state.set(AppState::MainMenu);
        }
        if unload_save_game_event.quit_mode == GameQuitMode::QuitToDesktop {
            app_exit_event_writer.send(AppExit);
        }
    }
}
