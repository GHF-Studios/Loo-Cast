pub mod components;
pub mod events;
pub mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::AppState;

use bevy::prelude::*;

pub struct SaveGamesMenuPlugin;

impl Plugin for SaveGamesMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<events::LoadSaveGameInstance>()
            .add_event::<events::DeleteSaveGameUI>()
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
                    handle_load_save_game_instance,
                    handle_delete_save_game_ui,
                )
                    .run_if(in_state(AppState::SaveGamesMenu)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::SaveGamesMenu), despawn_save_games_menu);
    }
}
