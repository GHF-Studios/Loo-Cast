use spacetime_engine_macros::define_workflow_mod;

pub mod spawn_player;

use bevy::prelude::*;

pub const NAME: &str = stringify!("Player");

pub struct PlayerWorkflowPlugin;

impl bevy::prelude::Plugin for PlayerWorkflowPlugin {
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
        name: stringify!("Player"),
        workflow_types: vec![
            spawn_player::Type::create_workflow(),
        ],
    });
}
