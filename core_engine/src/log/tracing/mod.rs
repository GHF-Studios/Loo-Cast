pub mod functions;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use systems::*;

#[macro_export]
macro_rules! traced {
    ($system:ident) => {
        |$crate_args| {
            let _span = tracing::info_span!(stringify!($system)).entered();
            $system($crate_args)
        }
    };
}

pub(crate) struct TracingPlugin;
impl Plugin for TracingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (flush_span_event_buffer_system, flush_log_event_buffer_system));
    }
}
