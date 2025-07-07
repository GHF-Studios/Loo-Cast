use bevy::prelude::*;

use crate::log_NEW::types::{LogLevel, SpanPathSelection, ModulePathSelection, PhysicalPathSelection};
use crate::log_NEW::ui::types::FilterTreeMode;

#[derive(Resource)]
pub struct LogViewerState {
    pub split_ratio: f32,
    pub threshold: LogLevel,
    pub tree_mode: FilterTreeMode,
    pub span_selections: Vec<SpanPathSelection>,
    pub module_selections: Vec<ModulePathSelection>,
    pub physical_selections: Vec<PhysicalPathSelection>
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