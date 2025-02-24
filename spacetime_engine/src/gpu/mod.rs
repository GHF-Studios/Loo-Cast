pub mod resources;

pub mod workflows;
pub mod vorkflows;
pub mod vorkflows_expanded;

use bevy::prelude::*;
use resources::*;

pub(in crate) struct GpuPlugin;
impl Plugin for GpuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShaderRegistry::default());
    }
}