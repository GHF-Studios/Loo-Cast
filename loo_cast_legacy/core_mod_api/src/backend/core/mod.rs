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

use orchestration::{AppSet, configure_app_sets};
use protocol::{AppOrchestrationSignal, AppOrchestrationState, OrchestrationFieldKind, OrchestrationPressure, PlayerMotionIntent};
use resources::EntityProxyRuntimeState;
use run_conditions::run_after_startup_finished;
use systems::{
    advance_entity_proxy_revision_system, enforce_root_transform_contract_system, ensure_entity_proxy_links_system, startup_system,
    sync_logic_proxies_from_main_entities_system, validate_entity_proxy_links_system,
};
use types::{Diegetic, Meta, ShortTime};

pub(crate) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        configure_app_sets(app);

        app.add_systems(Startup, startup_system)
            .add_systems(
                Update,
                (
                    advance_entity_proxy_revision_system.in_set(AppSet::Simulation),
                    ensure_entity_proxy_links_system
                        .in_set(AppSet::Simulation)
                        .after(advance_entity_proxy_revision_system),
                    validate_entity_proxy_links_system
                        .in_set(AppSet::Simulation)
                        .after(ensure_entity_proxy_links_system),
                    sync_logic_proxies_from_main_entities_system
                        .in_set(AppSet::Simulation)
                        .after(validate_entity_proxy_links_system),
                    enforce_root_transform_contract_system
                        .in_set(AppSet::Simulation)
                        .after(sync_logic_proxies_from_main_entities_system),
                )
                    .run_if(run_after_startup_finished),
            )
            .init_resource::<AppOrchestrationState>()
            .init_resource::<PlayerMotionIntent>()
            .init_resource::<EntityProxyRuntimeState>()
            .add_message::<AppOrchestrationSignal>()
            .register_type::<AppOrchestrationState>()
            .register_type::<PlayerMotionIntent>()
            .register_type::<EntityProxyRuntimeState>()
            .register_type::<OrchestrationPressure>()
            .register_type::<OrchestrationFieldKind>()
            .register_type::<AppOrchestrationSignal>()
            .register_type::<ShortTime>()
            .register_type::<Diegetic>()
            .register_type::<Meta>();
    }
}
