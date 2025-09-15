pub mod constants;
pub mod functions;
pub mod resources;
pub mod run_conditions;
pub mod statics;
pub mod types;

pub mod workflows;

use core_lib_macros::{composite_workflow, composite_workflow_return};
use bevy::prelude::*;

use crate::config::statics::CONFIG;
use crate::workflow::functions::handle_composite_workflow_return_later;

pub(crate) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
    }
}

#[tracing::instrument(skip_all)]
fn startup_system() {
    let handle = composite_workflow!(Startup, {
        warn!("Running composite workflow 'Startup'");

        workflow!(Camera::SpawnMainCameras);

        let chunk_shader_name = "texture_generators/example_compute_uv";
        let chunk_shader_path = "assets/core_lib/shaders/texture_generators/example_compute_uv.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: chunk_shader_name,
                shader_path: chunk_shader_path,
            }
        );

        if CONFIG.get::<bool>("debug/spawn_debug_objects") {
            workflow!(Debug::SpawnDebugObjects);
        }

        workflow!(Core::FinishStartup);
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();
        
        warn!("Finished composite workflow 'Startup'");
    });
}

// OLD SNIPPETS
// let chunk_coords: Vec<(i32, i32)> = (-8..=8)
//     .flat_map(|x| (-8..=8).map(move |y| (x, y)))
//     .collect();
// let texture_size = crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize;
// let param_data: Vec<Vec<f32>> = chunk_coords
//     .iter()
//     .map(|_| vec![0.0])
//     .collect();
// let texture_output = workflow!(IO, Gpu::GenerateTextures, Input {
//     shader_name: chunk_shader_name,
//     texture_sizes: vec![texture_size; chunk_coords.len()],
//     param_data,
// });
// let spawn_inputs: Vec<_> = chunk_coords
//     .into_iter()
//     .zip(texture_output.texture_handles.into_iter())
//     .map(|(chunk_coord, texture_handle)| crate::chunk::workflows::chunk::spawn_chunks::user_items::SpawnChunkInput {
//         chunk_coord,
//         chunk_owner_id: None,
//         metric_texture: texture_handle,
//     })
//     .collect();
// workflow!(IOE, Chunk::SpawnChunks, Input {
//     inputs: spawn_inputs
// });