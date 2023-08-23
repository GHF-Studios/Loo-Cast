pub mod components;
pub mod events;
pub mod resources;
pub mod structs;
mod systems;

use events::*;
use resources::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

pub struct SaveGamePlugin;

impl Plugin for SaveGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize Resources
            .init_resource::<SaveGameManager>()
            // Initialize Events
            .add_event::<CreateSaveGameEvent>()
            .add_event::<DeleteSaveGameEvent>()
            .add_event::<LoadSaveGameEvent>()
            // Enter State Systems
            // Update Systems
            .add_systems(
                Update,
                (handle_delete_save_game_event, handle_load_save_game_event)
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            .add_systems(
                Update,
                (handle_create_save_game_event).run_if(
                    in_state(AppState::SaveGamesMenu)
                        .or_else(in_state(AppState::CreateSaveGameMenu)),
                ),
            );
            // Exit State Systems
    }
}
