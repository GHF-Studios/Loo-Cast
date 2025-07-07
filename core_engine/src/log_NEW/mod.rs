pub mod functions;
pub mod resources;
pub mod statics;
pub mod systems;
pub mod types;

pub mod tracing;
pub mod ui;

use bevy::prelude::*;

use crate::log_NEW::{statics::LOG_REGISTRY_HANDLE, ui::resources::{ToolbarState, LogViewerState}};

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LOG_REGISTRY_HANDLE.clone())
            .insert_resource(ToolbarState::default())
            .insert_resource(LogViewerState::default());
    }
}