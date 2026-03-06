pub mod components;
pub mod functions;
pub mod orchestration;
pub mod protocol;
pub mod resources;
pub mod run_conditions;
pub mod schedules;
pub mod statics;
pub mod systems;
pub mod types;

pub mod workflows;

use crate::bevy::prelude::*;

use orchestration::configure_app_sets;
use protocol::{AppOrchestrationSignal, AppOrchestrationState, OrchestrationFieldKind, OrchestrationPressure, PlayerMotionIntent};
use systems::startup_system;
use types::{Diegetic, Meta, ShortTime};

pub(crate) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        configure_app_sets(app);

        app.add_systems(Startup, startup_system)
            .init_resource::<AppOrchestrationState>()
            .init_resource::<PlayerMotionIntent>()
            .add_message::<AppOrchestrationSignal>()
            .register_type::<AppOrchestrationState>()
            .register_type::<PlayerMotionIntent>()
            .register_type::<OrchestrationPressure>()
            .register_type::<OrchestrationFieldKind>()
            .register_type::<AppOrchestrationSignal>()
            .register_type::<ShortTime>()
            .register_type::<Diegetic>()
            .register_type::<Meta>();
    }
}
