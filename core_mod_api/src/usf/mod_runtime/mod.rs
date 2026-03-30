pub mod chunk_surface;
pub mod surface_field;
pub mod surface_meshing;
pub mod surface_projection;

use crate::bevy::prelude::*;
use crate::core::{orchestration::AppSet, run_conditions::run_after_startup_finished};
use crate::time::run_conditions::run_if_not_paused;
use chunk_surface::{
    UsfDemoChunkStore, UsfDemoChunkVisual, UsfDemoHydrationWorkflowState, UsfDemoSettings, bind_chunk_demo_visuals_to_world_presentation_root_system,
    prune_chunk_demo_store_system, queue_chunk_demo_hydration_requests_system, run_chunk_demo_hydration_workflow_system,
    run_if_active_test_mod_content_enabled, sync_chunk_demo_visual_transforms_system,
};

pub(crate) struct UsfModRuntimePlugin;
impl Plugin for UsfModRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfDemoSettings>()
            .init_resource::<UsfDemoChunkStore>()
            .init_resource::<UsfDemoHydrationWorkflowState>()
            .add_systems(
                PostUpdate,
                (
                    queue_chunk_demo_hydration_requests_system.in_set(AppSet::Presentation),
                    run_chunk_demo_hydration_workflow_system
                        .in_set(AppSet::Presentation)
                        .after(queue_chunk_demo_hydration_requests_system),
                    bind_chunk_demo_visuals_to_world_presentation_root_system
                        .in_set(AppSet::Presentation)
                        .after(run_chunk_demo_hydration_workflow_system),
                    sync_chunk_demo_visual_transforms_system
                        .in_set(AppSet::Presentation)
                        .after(bind_chunk_demo_visuals_to_world_presentation_root_system),
                    prune_chunk_demo_store_system.in_set(AppSet::Diagnostics),
                )
                    .run_if(run_after_startup_finished.and(run_if_not_paused).and(run_if_active_test_mod_content_enabled)),
            )
            .register_type::<UsfDemoChunkVisual>()
            .register_type::<UsfDemoSettings>();
    }
}
