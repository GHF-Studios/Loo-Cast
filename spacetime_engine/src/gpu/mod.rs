pub mod resources;
pub mod systems;

pub mod actions;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub(in crate) struct GpuPlugin;
impl Plugin for GpuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShaderPipelineRegistry::default())
            .add_systems(Startup, gpu_startup_system);
    }
}