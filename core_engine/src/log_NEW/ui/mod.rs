pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use crate::log_NEW::ui::resources::LogViewerState;

pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LogViewerState::default());
    }
}