pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use crate::log::ui::{resources::LogViewerState, systems::show_log_viewer_ui};

pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LogViewerState::default())
            .add_systems(Update, show_log_viewer_ui);
    }
}