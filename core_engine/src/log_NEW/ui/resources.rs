use bevy::prelude::*;

use crate::log_NEW::types::LogLevel;
use crate::log_NEW::ui::types::FilterTreeMode;

#[derive(Resource)]
pub struct LogViewerState {
    pub split_ratio: f32,
    pub threshold: LogLevel,
    pub tree_mode: FilterTreeMode,
    pub selected_spans: SpanTreeSelection,
    pub selected_locations: LocationTreeSelection,
}

impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            selected_spans: SpanTreeSelection::default(),
            selected_locations: LocationTreeSelection::default(),
            split_ratio: 0.35,
            threshold: LogLevel::Warn,
            tree_mode: FilterTreeMode::Span,
        }
    }
}

#[derive(Resource, Default)]
pub struct UiWindows {
    pub show_log_viewer: bool,
}