mod systems;

use systems::*;

use crate::game::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(Startup, spawn_camera)
            // Update Systems
            .add_systems(Update, lerp_to_player
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
                );
    }
}
