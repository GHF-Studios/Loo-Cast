mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use crate::game::SimulationState;

use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // Update Systems
            .add_systems(
                Update,
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::Paused)),
            )
            // Exit Systems
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
