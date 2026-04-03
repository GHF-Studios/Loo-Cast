pub mod field;
pub mod gpu_density;
pub mod intent;
pub mod meshing;
pub mod projection;
pub mod reconcile_workflow;
pub mod runtime;

use crate::bevy::prelude::*;
use crate::core::{orchestration::AppSet, run_conditions::run_after_startup_finished};
use crate::rhai_binding::bridges::domains::core_mod_api::usf::output_channels::{
    ChunkRealizationAudioEmitter, ChunkRealizationChannelAppliedEvent, ChunkRealizationChannelTelemetry, ChunkRealizationChannelTelemetrySettings,
    ChunkRealizationInteractionTrigger, ChunkRealizationParticleEmitter, ChunkRealizationSimulationService, OutputChannelAuthorityMode,
    OutputChannelExecutionRegistration, OutputChannelPayload, OutputChannelRegistry, report_chunk_realization_channel_telemetry_system,
};
use crate::time::run_conditions::run_if_not_paused;
use intent::{ChunkRealizationIntentGrace, sync_chunk_realization_intents_system};
use runtime::{
    ChunkRealizationCache, ChunkRealizationInstance, ChunkRealizationIntent, ChunkRealizationReconcileWorkflowState, UsfChunkRealizationRuntimeSettings,
    bind_chunk_realization_instances_to_world_presentation_root_system, clear_unbound_chunk_realization_instances_system, prune_chunk_realization_cache_system,
    queue_chunk_realization_reconcile_requests_system, run_chunk_realization_reconcile_workflow_system, run_if_chunk_realization_runtime_enabled,
    sync_chunk_realization_instance_transforms_system, validate_chunk_realization_channel_registrations_system,
};

pub(crate) struct ChunkRealizationPlugin;
impl Plugin for ChunkRealizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfChunkRealizationRuntimeSettings>()
            .init_resource::<ChunkRealizationCache>()
            .init_resource::<ChunkRealizationReconcileWorkflowState>()
            .init_resource::<OutputChannelRegistry>()
            .init_resource::<ChunkRealizationChannelTelemetrySettings>()
            .init_resource::<ChunkRealizationChannelTelemetry>()
            .add_message::<ChunkRealizationChannelAppliedEvent>()
            .add_systems(Startup, validate_chunk_realization_channel_registrations_system.in_set(AppSet::Diagnostics))
            .add_systems(
                PostUpdate,
                (
                    sync_chunk_realization_intents_system.in_set(AppSet::Presentation),
                    queue_chunk_realization_reconcile_requests_system.in_set(AppSet::Presentation),
                    run_chunk_realization_reconcile_workflow_system
                        .in_set(AppSet::Presentation)
                        .after(queue_chunk_realization_reconcile_requests_system)
                        .after(sync_chunk_realization_intents_system),
                    bind_chunk_realization_instances_to_world_presentation_root_system
                        .in_set(AppSet::Presentation)
                        .after(run_chunk_realization_reconcile_workflow_system),
                    clear_unbound_chunk_realization_instances_system
                        .in_set(AppSet::Presentation)
                        .after(sync_chunk_realization_intents_system),
                    sync_chunk_realization_instance_transforms_system
                        .in_set(AppSet::Presentation)
                        .after(bind_chunk_realization_instances_to_world_presentation_root_system),
                    prune_chunk_realization_cache_system.in_set(AppSet::Diagnostics),
                )
                    .run_if(run_after_startup_finished.and(run_if_not_paused).and(run_if_chunk_realization_runtime_enabled)),
            )
            .add_systems(
                PostUpdate,
                report_chunk_realization_channel_telemetry_system
                    .in_set(AppSet::Diagnostics)
                    .run_if(run_after_startup_finished),
            )
            .register_type::<ChunkRealizationInstance>()
            .register_type::<ChunkRealizationIntent>()
            .register_type::<ChunkRealizationIntentGrace>()
            .register_type::<ChunkRealizationAudioEmitter>()
            .register_type::<ChunkRealizationParticleEmitter>()
            .register_type::<ChunkRealizationInteractionTrigger>()
            .register_type::<ChunkRealizationSimulationService>()
            .register_type::<OutputChannelAuthorityMode>()
            .register_type::<OutputChannelExecutionRegistration>()
            .register_type::<OutputChannelPayload>()
            .register_type::<OutputChannelRegistry>()
            .register_type::<ChunkRealizationChannelAppliedEvent>()
            .register_type::<ChunkRealizationChannelTelemetrySettings>()
            .register_type::<ChunkRealizationChannelTelemetry>()
            .register_type::<UsfChunkRealizationRuntimeSettings>();
    }
}
