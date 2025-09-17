use core_api_macros::{composite_workflow, composite_workflow_return};
use bevy::prelude::*;

use crate::config::statics::CONFIG;
use crate::workflow::functions::handle_composite_workflow_return_later;

#[tracing::instrument(skip_all)]
pub(super) fn startup_system() {
    let handle = composite_workflow!(Startup, {
        warn!("Running composite workflow 'Startup'");

        workflow!(Camera::SpawnMainCameras);

        let example_uv_shader_name = "texture_generators/example_uv";
        let example_uv_shader_path = "assets/core_api/shaders/texture_generators/example_uv.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: example_uv_shader_name,
                shader_path: example_uv_shader_path,
            }
        );

        let example_world_shader_name = "texture_generators/example_world";
        let example_world_shader_path = "assets/core_api/shaders/texture_generators/example_world.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: example_world_shader_name,
                shader_path: example_world_shader_path,
            }
        );

        if CONFIG().get::<bool>("debug/spawn_debug_objects") {
            workflow!(Debug::SpawnDebugObjects);
        }

        workflow!(Core::FinishStartup);
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();
        
        warn!("Finished composite workflow 'Startup'");
    });
}