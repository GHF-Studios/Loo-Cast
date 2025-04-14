use spacetime_engine_macros::define_workflow_mod;

pub mod despawn_chunk;
pub mod spawn_chunk;
pub mod transfer_chunk_ownership;

use bevy::prelude::*;

pub const NAME: &str = stringify!("Chunk");

pub struct ChunkWorkflowPlugin;

impl bevy::prelude::Plugin for ChunkWorkflowPlugin {
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
        name: stringify!("Chunk"),
        workflow_types: vec![
            spawn_chunk::TypeIE::create_workflow(),
            despawn_chunk::TypeIE::create_workflow(),
            transfer_chunk_ownership::TypeIE::create_workflow(),
        ],
    });
}
