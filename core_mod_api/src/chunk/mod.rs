pub mod components;
pub mod errors;
pub mod functions;
pub mod hooks;
pub mod resources;
pub mod systems;
pub mod types;

pub mod intent;
pub mod workflows;

use bevy::prelude::*;
use components::Chunk;
use errors::{DespawnError, SpawnError, TransferOwnershipError};
use intent::{ActionIntent, ActionPriority, ResolutionError, ResolutionWarning, ResolvedActionIntent, State};
use resources::{ActionIntentBuffer, ActionIntentCommitBuffer, ChunkManager, ChunkRenderExecutorRegistry, ChunkRenderHandles};
use systems::{chunk_startup_system, chunk_update_system, process_chunk_actions_system};
use types::ChunkActionWorkflowHandles;

use crate::{core::run_conditions::run_after_startup_finished, time::run_conditions::run_if_not_paused};

pub(crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionIntentBuffer::default())
            .insert_resource(ActionIntentCommitBuffer::default())
            .insert_resource(ChunkManager::default())
            .insert_resource(ChunkRenderExecutorRegistry::default())
            .add_systems(Startup, chunk_startup_system)
            .add_systems(Update, chunk_update_system.run_if(run_after_startup_finished.and(run_if_not_paused)))
            .add_systems(
                PostUpdate,
                process_chunk_actions_system.run_if(run_after_startup_finished.and(run_if_not_paused)),
            )
            .register_type::<Chunk>()
            .register_type::<ActionIntentBuffer>()
            .register_type::<ActionIntentCommitBuffer>()
            .register_type::<ChunkManager>()
            .register_type::<ChunkRenderHandles>()
            .register_type::<State>()
            .register_type::<ActionIntent>()
            .register_type::<ResolutionError>()
            .register_type::<ResolutionWarning>()
            .register_type::<ResolvedActionIntent>()
            .register_type::<SpawnError>()
            .register_type::<DespawnError>()
            .register_type::<TransferOwnershipError>()
            .register_type::<ActionPriority>()
            .register_type::<ChunkActionWorkflowHandles>();
    }
}
