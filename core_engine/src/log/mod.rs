pub mod arena;
pub mod functions;
pub mod location_tree;
pub mod resources;
pub mod span_tree;
pub mod statics;
pub mod storage;
pub mod systems;
pub mod traits;
pub mod types;

use bevy::prelude::*;

use crate::log::{resources::*, statics::LOG_STORAGE_HANDLE, systems::*};

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LOG_STORAGE_HANDLE.clone())
            .insert_resource(UiWindows::default())
            .init_resource::<LogViewerState>()
            .add_systems(Update, (show_toolbar_ui, show_log_viewer_ui));
    }
}
