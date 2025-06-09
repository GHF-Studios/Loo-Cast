pub mod bundles;
pub mod components;
pub mod errors;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod intent;
pub mod workflows;

use bevy::prelude::*;
use resources::{ActionIntentBuffer, ActionIntentCommitBuffer, ChunkManager};
use systems::{chunk_startup_system, chunk_update_system, process_chunk_actions_system, chunk_debug_log_system};

pub(crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionIntentBuffer::default())
            .insert_resource(ActionIntentCommitBuffer::default())
            .insert_resource(ChunkManager::default())
            .add_systems(Startup, chunk_startup_system)
            .add_systems(Update, chunk_update_system)
            .add_systems(PostUpdate, process_chunk_actions_system)
            .add_systems(Update, chunk_debug_log_system);
    }
}
