pub mod resources;
pub mod statics;
pub mod types;

pub mod selection;
pub mod tracing;
pub mod ui;

use bevy::prelude::*;

use crate::log_NEW::statics::LOG_REGISTRY_HANDLE;

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LOG_REGISTRY_HANDLE.clone());
    }
}