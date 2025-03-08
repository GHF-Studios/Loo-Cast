pub mod resources;

pub mod workflows;
pub mod vorkflows;

use bevy::prelude::*;
use resources::*;

pub(in crate) struct GpuPlugin;
impl Plugin for GpuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShaderRegistry::default());
    }
}