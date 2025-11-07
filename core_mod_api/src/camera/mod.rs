pub mod components;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod workflows;

use bevy::prelude::*;
use components::{MainCamera, UiCamera};
use systems::{reserve_camera_entities, setup_main_render_target, main_camera_zoom_system, update_view_scale_from_zoom};
use resources::{ZoomFactor, ViewScale};

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .insert_resource(ZoomFactor::default())
            .insert_resource(ViewScale::default())
            .add_systems(PreStartup, (
                reserve_camera_entities.before(setup_main_render_target),
                setup_main_render_target,
            ))
            .add_systems(Update, (main_camera_zoom_system.before(update_view_scale_from_zoom), update_view_scale_from_zoom).run_if(run_after_startup_finished))
            .register_type::<MainCamera>()
            .register_type::<UiCamera>()
            .register_type::<ZoomFactor>()
            .register_type::<ViewScale>();
    }
}
