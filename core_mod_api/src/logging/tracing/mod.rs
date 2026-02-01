pub mod functions;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use systems::*;
use types::LogTreeTracingLayer;

use crate::core::run_conditions::run_after_startup_finished;

pub(crate) struct TracingPlugin;
impl Plugin for TracingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flush_span_message_buffer_system, flush_log_message_buffer_system).run_if(run_after_startup_finished),
        )
        .register_type::<LogTreeTracingLayer>();
    }
}
