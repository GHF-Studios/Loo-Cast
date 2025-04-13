use spacetime_engine_macros::define_workflow_mod;

pub mod spawn_main_camera;

pub const NAME: &str = stringify!("Camera");

pub struct CameraWorkflowPlugin;

impl bevy::prelude::Plugin for CameraWorkflowPlugin {
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
        name: stringify!("Camera"),
        workflow_types: vec![spawn_main_camera::Type::create_workflow()],
    });
}
