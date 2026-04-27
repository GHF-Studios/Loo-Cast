use crate::bevy::prelude::*;

use crate::logging::types::LogLevel;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct LogViewerState {
    pub split_ratio: f32,
    pub threshold: LogLevel,
}
impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            split_ratio: 0.35,
            threshold: LogLevel::Warn,
        }
    }
}
