use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ToolbarState {
    pub show_perf_ui: bool,
    pub show_log_viewer_ui: bool,
    pub show_chunk_manager_debug_ui: bool,
}