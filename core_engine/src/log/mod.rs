pub mod functions;
pub mod resources;
pub mod statics;
pub mod systems;
pub mod types;

pub mod tracing;
pub mod ui;

use bevy::prelude::*;

use crate::log::{statics::LOG_REGISTRY_HANDLE};

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ui::UiPlugin)
            .insert_resource(LOG_REGISTRY_HANDLE.clone());
    }
}