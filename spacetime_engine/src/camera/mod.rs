pub mod components;
pub mod systems;
pub mod workflows;

pub mod workflows_MACROINPUT;

use bevy::prelude::*;
use systems::main_camera_zoom_system;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .add_systems(Update, main_camera_zoom_system);
    }
}
