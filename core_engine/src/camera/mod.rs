pub mod components;
pub mod systems;
pub mod types;
pub mod workflows;

use bevy::prelude::*;
use components::MainCamera;
use systems::main_camera_zoom_system;
use types::ZoomFactor;

use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .add_systems(Update, main_camera_zoom_system.run_if(run_if_not_paused))
            .register_type::<MainCamera>()
            .register_type::<ZoomFactor>();
    }
}
