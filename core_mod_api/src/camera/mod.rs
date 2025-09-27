pub mod components;
pub mod resources;
pub mod systems;
pub mod types;
pub mod workflows;

use bevy::prelude::*;
use components::MainCamera;
use systems::{main_camera_zoom_system, setup_main_render_target};
use types::ZoomFactor;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .insert_resource(ZoomFactor::default())
            .add_systems(PreStartup, setup_main_render_target)
            .add_systems(Update, main_camera_zoom_system.run_if(run_after_startup_finished))
            .register_type::<MainCamera>()
            .register_type::<ZoomFactor>();
    }
}
