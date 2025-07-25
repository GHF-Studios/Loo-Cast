pub mod functions;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use systems::*;

pub(crate) struct TracingPlugin;
impl Plugin for TracingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (flush_span_event_buffer_system, flush_log_event_buffer_system));
    }
}