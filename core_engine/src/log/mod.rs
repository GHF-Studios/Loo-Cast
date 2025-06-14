pub mod arena;
pub mod functions;
pub mod resources;
pub mod statics;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use crate::log::{resources::LogViewerState, statics::LOG_TREE_HANDLE, systems::show_debug_ui};

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LOG_TREE_HANDLE.clone())
            .init_resource::<LogViewerState>()
            .add_systems(Update, show_debug_ui);
    }
}
