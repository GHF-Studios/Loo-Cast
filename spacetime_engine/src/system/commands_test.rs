use crate::system::AppState;
use spacetime_engine_derive::*;

use bevy::prelude::*;

pub struct CommandsTestPlugin;

#[derive(Resource)]
pub struct CommandsTestManager {}

impl Plugin for CommandsTestPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(CommandsTestManager {})
            // Enter Systems
            .add_systems(OnEnter(AppState::MainMenu), CommandsTestManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (
                    CommandsTestManager::update,
                )
                    .run_if(in_state(AppState::MainMenu))
            )
            // Exit Systems
            .add_systems(OnExit(AppState::MainMenu), CommandsTestManager::terminate);
    }
}

impl CommandsTestManager {
    fn initialize() {
        Test::hello_macro();
    }

    fn update() {

    }

    fn terminate() {

    }
}

pub trait HelloMacro {
    fn hello_macro();
}


#[derive(HelloMacro)]
struct Test;
