mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::AppState;

use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // Update Systems
            .add_systems(Update, 
                (interact_with_play_button, interact_with_quit_button).run_if(in_state(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu),despawn_main_menu);
    }
}
