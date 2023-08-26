use crate::game::SimulationState;
use crate::game_config::events::*;
use crate::game_state::events::*;
use crate::save_game::enums::GameQuitMode;
use crate::save_game::events::*;
use crate::universe::events::*;
use crate::AppState;

use super::resources::GameManager;

use bevy::app::AppExit;
use bevy::prelude::*;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

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

pub fn handle_confirm_loaded_save_game_event(
    mut commands: Commands,
    mut confirm_loaded_save_game_event_reader: EventReader<ConfirmLoadedSaveGame>,
    mut load_game_config_event_writer: EventWriter<LoadGameConfig>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Some(confirm_loaded_save_game_event) =
        confirm_loaded_save_game_event_reader.iter().last()
    {
        commands.insert_resource(GameManager {
            current_save_game: confirm_loaded_save_game_event.save_game.clone(),
        });
        app_state_next_state.set(AppState::Game);
        println!(
            "Loaded Save Game '{}'.",
            confirm_loaded_save_game_event.save_game.name
        );
        load_game_config_event_writer.send(LoadGameConfig {});
    }
}

pub fn handle_confirm_loaded_game_config(
    mut confirm_loaded_game_config_event_reader: EventReader<ConfirmLoadedGameConfig>,
    mut load_game_state_event_writer: EventWriter<LoadGameState>,
) {
    if let Some(_) = confirm_loaded_game_config_event_reader.iter().last() {
        println!("Loaded Game Config.");
        load_game_state_event_writer.send(LoadGameState {});
    }
}

pub fn handle_confirm_loaded_game_state(
    mut confirm_loaded_game_state_event_reader: EventReader<ConfirmLoadedGameState>,
    mut load_universe_event_writer: EventWriter<LoadUniverse>,
) {
    if let Some(_) = confirm_loaded_game_state_event_reader.iter().last() {
        println!("Loaded Game State.");
        load_universe_event_writer.send(LoadUniverse {});
    }
}

pub fn handle_confirm_loaded_universe(
    mut confirm_loaded_universe_event_reader: EventReader<ConfirmLoadedUniverse>,
) {
    if let Some(_) = confirm_loaded_universe_event_reader.iter().last() {
        println!("Loaded Universe.");
    }
}

pub fn handle_confirm_unloaded_save_game_event(
    mut commands: Commands,
    mut confirm_unloaded_save_game_event_reader: EventReader<ConfirmUnloadedSaveGame>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    game_manager: ResMut<GameManager>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Some(confirm_unloaded_save_game_event) =
        confirm_unloaded_save_game_event_reader.iter().last()
    {
        if confirm_unloaded_save_game_event.quit_mode == GameQuitMode::QuitToMainMenu {
            app_state_next_state.set(AppState::MainMenu);
            println!(
                "Unloaded Save Game '{}' to Main Menu.",
                game_manager.current_save_game.name
            );
        }
        if confirm_unloaded_save_game_event.quit_mode == GameQuitMode::QuitToDesktop {
            app_exit_event_writer.send(AppExit);
            println!(
                "Unloaded Save Game '{}' to Desktop.",
                game_manager.current_save_game.name
            );
        }
        commands.remove_resource::<GameManager>();
    }
}
