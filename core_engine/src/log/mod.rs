pub mod functions;
pub mod resources;
pub mod statics;
pub mod systems;
pub mod types;

pub mod tracing;
pub mod ui;

use bevy::prelude::*;

use crate::log::resources::LogRegistry;

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((tracing::TracingPlugin, ui::UiPlugin))
            .insert_resource(LogRegistry::default());
    }
}