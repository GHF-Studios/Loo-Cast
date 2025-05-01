pub mod bundles;
pub mod components;
pub mod enums;
pub mod errors;
pub mod functions;
pub mod resources;
pub mod systems;

pub mod workflows;

// pub mod workflows_MACROINPUT;
// pub mod workflows_MACROOUTPUT;

use bevy::prelude::*;
use resources::{ChunkManager, ChunkWorkflowBuffer};
use systems::{process_chunk_workflows_system, chunk_startup_system, chunk_update_system};

pub(crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkWorkflowBuffer::default())
            .insert_resource(ChunkManager::default())
            .add_systems(Startup, chunk_startup_system)
            .add_systems(Update, chunk_update_system)
            .add_systems(PostUpdate, process_chunk_workflows_system);
    }
}
