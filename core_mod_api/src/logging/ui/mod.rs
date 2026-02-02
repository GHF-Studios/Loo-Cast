pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

use crate::bevy::prelude::*;
use resources::LogViewerState;
use systems::show_log_viewer_ui;
use types::SelectionMode;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LogViewerState::default())
            .add_systems(Update, show_log_viewer_ui.run_if(run_after_startup_finished))
            .register_type::<LogViewerState>()
            .register_type::<SelectionMode>();
    }
}
