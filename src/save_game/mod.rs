pub mod components;
pub mod enums;
pub mod events;
pub mod resources;
pub mod structs;
mod systems;

use events::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct SaveGamePlugin;

impl Plugin for SaveGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<CreateSaveGame>()
            .add_event::<DeleteSaveGame>()
            // Startup Systems
            .add_systems(Startup, init)
            // Enter State Systems
            // Update Systems
            .add_systems(
                Update,
                (handle_delete_save_game)
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            .add_systems(
                Update,
                (handle_create_save_game).run_if(
                    in_state(AppState::CreateSaveGameMenu),
                ),
            );
        // Exit State Systems
    }
}
