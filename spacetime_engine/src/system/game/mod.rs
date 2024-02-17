use bevy::prelude::*;
use crate::system::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Paused,
    Running,
}

#[derive(Event)]
pub struct EnterGame;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EnterGame>()
            // States
            .add_state::<GameState>()
            // Startup Systems
            .add_systems(PreStartup, GameManager::pre_startup)
            .add_systems(Startup, GameManager::startup)
            // Update Systems
            .add_systems(Update, GameManager::resume_game.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Paused)))
            .add_systems(Update, GameManager::pause_game.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))
            .add_systems(Update, GameManager::handle_enter_game.run_if(in_state(AppState::Games)))
            .add_systems(Update, GameManager::handle_exit_game.run_if(in_state(AppState::Game)));
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

        if keyboard_input.just_pressed(KeyCode::Escape) {
            next_game_state.set(GameState::Running);

            info!("Resumed game.");
        }
    }

    fn pause_game(
        keyboard_input: Res<Input<KeyCode>>, 
        mut next_game_state: ResMut<NextState<GameState>>
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            next_game_state.set(GameState::Paused);
            
            info!("Paused game.");
        }
    }

    fn handle_enter_game(
        mut enter_game_event_reader: EventReader<EnterGame>,
        mut next_game_state: ResMut<NextState<GameState>>,
        mut next_app_state: ResMut<NextState<AppState>>,
    ) {
        for _ in enter_game_event_reader.iter() {
            next_game_state.set(GameState::Paused);
            next_app_state.set(AppState::Game);
            
            info!("Entered game.");
        }
    }

    fn handle_exit_game(
        keyboard_input: Res<Input<KeyCode>>, 
        mut next_app_state: ResMut<NextState<AppState>>,
    ) {

        if keyboard_input.just_pressed(KeyCode::ControlLeft) && keyboard_input.just_pressed(KeyCode::Delete) {
            next_app_state.set(AppState::Games);
            
            info!("Exited game.");
        }
    }
}