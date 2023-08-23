pub mod components;
pub mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::AppState;

use bevy::prelude::*;

pub struct CreateSaveGameMenuPlugin;

impl Plugin for CreateSaveGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter State Systems
            .add_systems(OnEnter(AppState::CreateSaveGameMenu), spawn_create_save_game_menu)
            // Update Systems
            .add_systems(
                Update,
                (
                    interact_with_cancel_create_save_game_button,
                    interact_with_confirm_create_save_game_button,
                ).run_if(in_state(AppState::CreateSaveGameMenu)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::CreateSaveGameMenu), despawn_create_save_game_menu);
    }
}