use spacetime_engine_macros::define_workflow_mod;

pub mod setup_texture_generator;
pub mod generate_texture;

use bevy::prelude::*;

pub const NAME: &str = stringify!("Gpu");

pub struct GpuWorkflowPlugin;

impl bevy::prelude::Plugin for GpuWorkflowPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(bevy::prelude::PreStartup, register_workflow_type_module);
    }
}
fn register_workflow_type_module(
    mut workflow_type_module_registry: bevy::prelude::ResMut<
        crate::workflow::resources::WorkflowTypeModuleRegistry,
    >,
) {
    workflow_type_module_registry.register(crate::workflow::types::WorkflowTypeModule {
        name: stringify!("Gpu"),
        workflow_types: vec![
            setup_texture_generator::TypeIE::create_workflow(),
            generate_texture::TypeIOE::create_workflow(),
        ],
    });
}
