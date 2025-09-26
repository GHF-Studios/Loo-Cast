pub mod external;
pub mod resources;

pub mod workflows;

use bevy::prelude::*;
use resources::ShaderRegistry;

pub(crate) struct GpuPlugin;
impl Plugin for GpuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShaderRegistry::default()).register_type::<ShaderRegistry>();
    }
}
