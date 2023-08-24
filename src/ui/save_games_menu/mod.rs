pub mod components;
pub mod events;
pub mod styles;
mod systems;

use systems::event_handler::*;
use systems::interactions::*;
use systems::layout::*;

use crate::AppState;

use bevy::prelude::*;

pub struct SaveGamesMenuPlugin;

impl Plugin for SaveGamesMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(OnEnter(AppState::SaveGamesMenu), spawn_save_games_menu)
            // Update Systems
            .add_systems(
                Update,
                (
                    interact_with_back_to_main_menu_button,
                    interact_with_create_save_game_button,
                    interact_with_delete_save_game_button,
                    interact_with_load_save_game_button,
                )
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            .add_systems(
                Update,
                (handle_created_save_game, handle_deleted_save_game).run_if(
                    in_state(AppState::SaveGamesMenu)
                        .or_else(in_state(AppState::CreateSaveGameMenu)),
                ),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::SaveGamesMenu), despawn_save_games_menu);
    }
}
