pub mod components;
pub mod demo;
pub mod enums;
pub mod errors;
pub mod functions;
pub mod gpu_density;
pub mod messages;
pub mod resources;
pub mod run_conditions;
pub mod systems;
pub mod types;

pub mod workflows;

use crate::bevy::prelude::*;
use components::{Chunk, ChunkActor, ChunkDebugWireframe, ChunkLoader, PhenomenonFrontierView};
use demo::{UsfDemoChunkStore, UsfDemoChunkVisual, UsfDemoHydrationWorkflowState, UsfDemoSettings};
use enums::ZoomState;
use errors::{DespawnError, SpawnError};
use messages::ChunkBatchLifecycleMessage;
use resources::{ChunkActionWorkflowState, ChunkBatchTracker, ChunkLoadGate, ChunkLoadGateLockInfo, ChunkLoadGateState, ChunkManager};
use systems::{
    chunk_detection_system, chunk_management_system, chunk_timeout_signal_system, chunk_zoom_cooldown_system, sync_chunk_orchestration_state_system,
};

use crate::{
    core::{orchestration::AppSet, run_conditions::run_after_startup_finished},
    time::run_conditions::run_if_not_paused,
};

pub(crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChunkCorePlugin);
        app.add_plugins(ChunkPlaceholderContentPlugin);
    }
}

pub(crate) struct ChunkCorePlugin;
impl Plugin for ChunkCorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
            .insert_resource(ChunkLoadGate::default())
            .insert_resource(ChunkBatchTracker::default())
            .insert_resource(ChunkActionWorkflowState::default())
            .add_message::<ChunkBatchLifecycleMessage>()
            .add_systems(PreUpdate, chunk_timeout_signal_system.run_if(run_after_startup_finished))
            .add_systems(
                Update,
                chunk_zoom_cooldown_system
                    .in_set(AppSet::Simulation)
                    .run_if(run_after_startup_finished.and(run_if_not_paused)),
            )
            .add_systems(
                PostUpdate,
                (
                    demo::sync_chunk_manager_loader_state_system,
                    chunk_detection_system.pipe(chunk_management_system),
                )
                    .chain()
                    .in_set(AppSet::ChunkOrchestration)
                    .run_if(run_after_startup_finished.and(run_if_not_paused)),
            )
            .add_systems(
                PostUpdate,
                sync_chunk_orchestration_state_system
                    .in_set(AppSet::Diagnostics)
                    .run_if(run_after_startup_finished),
            )
            .register_type::<Chunk>()
            .register_type::<ChunkDebugWireframe>()
            .register_type::<ChunkActor>()
            .register_type::<ChunkLoader>()
            .register_type::<PhenomenonFrontierView>()
            .register_type::<ChunkManager>()
            .register_type::<ChunkLoadGate>()
            .register_type::<ChunkLoadGateState>()
            .register_type::<ChunkLoadGateLockInfo>()
            .register_type::<SpawnError>()
            .register_type::<DespawnError>()
            .register_type::<ZoomState>();
    }
}

pub(crate) struct ChunkPlaceholderContentPlugin;
impl Plugin for ChunkPlaceholderContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfDemoSettings>()
            .init_resource::<UsfDemoChunkStore>()
            .init_resource::<UsfDemoHydrationWorkflowState>()
            .add_systems(
                PostUpdate,
                (
                    demo::queue_chunk_demo_hydration_requests_system.in_set(AppSet::Presentation),
                    demo::run_chunk_demo_hydration_workflow_system
                        .in_set(AppSet::Presentation)
                        .after(demo::queue_chunk_demo_hydration_requests_system),
                    demo::bind_chunk_demo_visuals_to_world_presentation_root_system
                        .in_set(AppSet::Presentation)
                        .after(demo::run_chunk_demo_hydration_workflow_system),
                    demo::sync_chunk_demo_visual_transforms_system
                        .in_set(AppSet::Presentation)
                        .after(demo::bind_chunk_demo_visuals_to_world_presentation_root_system),
                    demo::prune_chunk_demo_store_system.in_set(AppSet::Diagnostics),
                )
                    .run_if(
                        run_after_startup_finished
                            .and(run_if_not_paused)
                            .and(demo::run_if_placeholder_gameplay_content_enabled),
                    ),
            )
            .register_type::<UsfDemoChunkVisual>()
            .register_type::<UsfDemoSettings>();
    }
}
