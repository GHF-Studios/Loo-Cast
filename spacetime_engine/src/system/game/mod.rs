use bevy::prelude::*;
use crate::system::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Paused,
    Running,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<GameState>()
            // Startup Systems
            .add_systems(PreStartup, GameManager::pre_startup)
            .add_systems(Startup, GameManager::startup)
            // Update Systems
            .add_systems(Update, GameManager::resume_game.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Paused)))
            .add_systems(Update, GameManager::pause_game.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))
            .add_systems(Update, GameManager::exit_game.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Paused)));
    }
}

#[derive(Resource, Default)]
pub struct GameManager {
}

impl GameManager {
    fn pre_startup(mut commands: Commands) {
        info!("Pre-Starting game Manager...");

        commands.insert_resource(GameManager::default());

        info!("Pre-Started game Manager.");
    }

    fn startup(commands: Commands) {
        info!("Starting game Manager...");

        info!("Started game Manager.");
    }

    fn resume_game(
        keyboard_input: Res<Input<KeyCode>>, 
        mut next_game_state: ResMut<NextState<GameState>>
    ) {
        info!("Resuming game...");

        if keyboard_input.pressed(KeyCode::Escape) {
            next_game_state.set(GameState::Running);
        }

        info!("Resumed game.");
    }

    fn pause_game(
        keyboard_input: Res<Input<KeyCode>>, 
        mut next_game_state: ResMut<NextState<GameState>>
    ) {
        info!("Pausing game...");

        if keyboard_input.pressed(KeyCode::Escape) {
            next_game_state.set(GameState::Paused);
        }

        info!("Paused game.");
    }

    fn exit_game(
        keyboard_input: Res<Input<KeyCode>>, 
        mut next_app_state: ResMut<NextState<AppState>>,
    ) {
        info!("Exiting game...");

        if keyboard_input.pressed(KeyCode::Return) {
            next_app_state.set(AppState::MainMenu);
        }

        info!("Exited game.");
    }
}