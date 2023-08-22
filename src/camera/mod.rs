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
            .add_startup_system(spawn_camera)
            // OnUpdate Systems
            .add_system(
                lerp_to_player
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
