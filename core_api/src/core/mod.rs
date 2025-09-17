pub mod constants;
pub mod functions;
pub mod resources;
pub mod run_conditions;
pub mod statics;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;
use systems::startup_system;

pub(crate) struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
    }
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