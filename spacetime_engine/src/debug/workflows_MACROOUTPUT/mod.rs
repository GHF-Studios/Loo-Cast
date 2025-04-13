use spacetime_engine_macros::define_workflow_mod;

pub mod spawn_debug_ui;
pub mod spawn_debug_objects;

use bevy::prelude::*;

pub const NAME: &str = stringify!("Debug");

pub struct DebugWorkflowPlugin;

impl bevy::prelude::Plugin for DebugWorkflowPlugin {
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
        name: stringify!("Debug"),
        workflow_types: vec![
            spawn_debug_ui::Type::create_workflow(),
            spawn_debug_objects::Type::create_workflow()
        ],
    });
}
