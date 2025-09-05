use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ToolbarState {
    pub enabled: bool,
    pub show_perf_ui: bool,
    pub show_log_viewer_ui: bool,
    pub show_chunk_manager_debug_ui: bool,
    pub show_log_registry_debug_ui: bool,
}
impl ToolbarState {
    pub fn enable(&mut self) {
        *self = Self {
            enabled: true,
            show_perf_ui: false,
            show_log_viewer_ui: false,
            show_chunk_manager_debug_ui: false,
            show_log_registry_debug_ui: false,
        };
    }

    pub fn disable_all(&mut self) {
        *self = Self {
            enabled: false,
            show_perf_ui: false,
            show_log_viewer_ui: false,
            show_chunk_manager_debug_ui: false,
            show_log_registry_debug_ui: false,
        };
    }
}
impl Default for ToolbarState {
    fn default() -> Self {
        Self {
            enabled: true,
            show_perf_ui: true,
            show_log_viewer_ui: false,
            show_chunk_manager_debug_ui: false,
            show_log_registry_debug_ui: false,
        }
    }
}
