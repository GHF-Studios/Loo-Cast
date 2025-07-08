use bevy::prelude::*;

use crate::log::types::{LogLevel, SpanPathSelections, ModulePathSelections, PhysicalPathSelections};
use crate::log::ui::types::FilterTreeMode;

#[derive(Resource)]
pub struct LogViewerState {
    pub split_ratio: f32,
    pub threshold: LogLevel,
    pub tree_mode: FilterTreeMode,
    pub span_selections: SpanPathSelections,
    pub module_selections: ModulePathSelections,
    pub physical_selections: PhysicalPathSelections
}
impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            split_ratio: 0.35,
            threshold: LogLevel::Warn,
            tree_mode: FilterTreeMode::Span,
            span_selections: Vec::new(),
            module_selections: Vec::new(),
            physical_selections: Vec::new(),
        }
    }
}