pub mod chunk_manifestation;
pub mod chunk_manifestation_binding;
pub mod manifestation_capability;
pub mod manifestation_field;
pub mod manifestation_meshing;
pub mod manifestation_projection;

use crate::bevy::prelude::*;
use crate::core::{orchestration::AppSet, run_conditions::run_after_startup_finished};
use crate::time::run_conditions::run_if_not_paused;
use chunk_manifestation::{
    ChunkManifestationBinding, UsfChunkManifestationHydrationWorkflowState, UsfChunkManifestationInstance, UsfChunkManifestationRuntimeSettings,
    UsfChunkManifestationStore, bind_chunk_manifestation_instances_to_world_presentation_root_system, clear_unbound_chunk_manifestation_instances_system,
    prune_chunk_manifestation_store_system, queue_chunk_manifestation_hydration_requests_system, run_chunk_manifestation_hydration_workflow_system,
    run_if_chunk_manifestation_runtime_enabled, sync_chunk_manifestation_instance_transforms_system, validate_chunk_manifestation_capability_contracts_system,
};
use chunk_manifestation_binding::{ChunkManifestationAuthorityGrace, sync_chunk_manifestation_bindings_system};

pub(crate) struct UsfRuntimePlugin;
impl Plugin for UsfRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfChunkManifestationRuntimeSettings>()
            .init_resource::<UsfChunkManifestationStore>()
            .init_resource::<UsfChunkManifestationHydrationWorkflowState>()
            .add_systems(Startup, validate_chunk_manifestation_capability_contracts_system.in_set(AppSet::Diagnostics))
            .add_systems(
                PostUpdate,
                (
                    sync_chunk_manifestation_bindings_system.in_set(AppSet::Presentation),
                    queue_chunk_manifestation_hydration_requests_system.in_set(AppSet::Presentation),
                    run_chunk_manifestation_hydration_workflow_system
                        .in_set(AppSet::Presentation)
                        .after(queue_chunk_manifestation_hydration_requests_system)
                        .after(sync_chunk_manifestation_bindings_system),
                    bind_chunk_manifestation_instances_to_world_presentation_root_system
                        .in_set(AppSet::Presentation)
                        .after(run_chunk_manifestation_hydration_workflow_system),
                    clear_unbound_chunk_manifestation_instances_system
                        .in_set(AppSet::Presentation)
                        .after(sync_chunk_manifestation_bindings_system),
                    sync_chunk_manifestation_instance_transforms_system
                        .in_set(AppSet::Presentation)
                        .after(bind_chunk_manifestation_instances_to_world_presentation_root_system),
                    prune_chunk_manifestation_store_system.in_set(AppSet::Diagnostics),
                )
                    .run_if(
                        run_after_startup_finished
                            .and(run_if_not_paused)
                            .and(run_if_chunk_manifestation_runtime_enabled),
                    ),
            )
            .register_type::<UsfChunkManifestationInstance>()
            .register_type::<ChunkManifestationBinding>()
            .register_type::<ChunkManifestationAuthorityGrace>()
            .register_type::<UsfChunkManifestationRuntimeSettings>();
    }
}
