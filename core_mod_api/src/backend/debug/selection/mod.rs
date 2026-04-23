pub mod systems;

use crate::bevy::prelude::*;
use systems::{draw_selection_highlight_gizmos, handle_selection};

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_selection, draw_selection_highlight_gizmos).chain().run_if(run_after_startup_finished),
        );
    }
}
