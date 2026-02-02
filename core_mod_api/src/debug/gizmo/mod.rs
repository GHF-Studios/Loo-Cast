pub mod components;
pub mod systems;
pub mod types;

use crate::bevy::prelude::*;
use components::{GizmoArrow, GizmoRoot};
use systems::{move_selected_with_gizmo, setup, update_gizmo_visibility_and_position};
use types::Axis2D;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct GizmoPlugin;
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (update_gizmo_visibility_and_position, move_selected_with_gizmo).run_if(run_after_startup_finished),
            )
            .register_type::<Axis2D>()
            .register_type::<GizmoRoot>()
            .register_type::<GizmoArrow>();
    }
}
