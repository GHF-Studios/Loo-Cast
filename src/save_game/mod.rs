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
            .add_event::<CreateSaveGame>()
            .add_event::<DeleteSaveGame>()
            .add_event::<LoadSaveGame>()
            .add_event::<ConfirmCreatedSaveGame>()
            .add_event::<ConfirmDeletedSaveGame>()
            .add_event::<ConfirmLoadedSaveGame>()
            // Enter State Systems
            // Update Systems
            .add_systems(
                Update,
                (handle_deleted_save_game, handle_loaded_save_game)
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            .add_systems(
                Update,
                (handle_create_save_game).run_if(
                    in_state(AppState::SaveGamesMenu)
                        .or_else(in_state(AppState::CreateSaveGameMenu)),
                ),
            );
        // Exit State Systems
    }
}
