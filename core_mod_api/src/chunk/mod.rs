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
use types::{ChunkActionWorkflowHandles, ChunkOwnerId};

use crate::core_mod_macros::configure_app_with_all_scales;
use crate::usf::scale::*;
use crate::{core::run_conditions::run_after_startup_finished, time::run_conditions::run_if_not_paused};

pub(crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRenderExecutorRegistry::default())
            .add_systems(PostUpdate, process_chunk_actions_system.run_if(run_after_startup_finished.and(run_if_not_paused)))
            .register_type::<SpawnError>()
            .register_type::<DespawnError>()
            .register_type::<TransferOwnershipError>()
            .register_type::<ActionPriority>()
            .register_type::<ChunkActionWorkflowHandles>();

        configure_app_with_all_scales!(
            { .insert_resource(ActionIntentBuffer::<__S__>::default()) },
            { .insert_resource(ActionIntentCommitBuffer::<__S__>::default()) },
            { .insert_resource(ChunkManager::<__S__>::default()) },

            { .add_systems(Startup, chunk_startup_system::<__S__>) },
            { .add_systems(Update, chunk_update_system::<__S__>.run_if(run_after_startup_finished.and(run_if_not_paused))) },

            { .register_type::<Chunk::<__S__>>() },
            { .register_type::<ActionIntentBuffer::<__S__>>() },
            { .register_type::<ActionIntentCommitBuffer::<__S__>>() },
            { .register_type::<ChunkManager::<__S__>>() },
            { .register_type::<ChunkRenderHandles::<__S__>>() },
            { .register_type::<State::<__S__>>() },
            { .register_type::<ActionIntent::<__S__>>() },
            { .register_type::<ResolutionError::<__S__>>() },
            { .register_type::<ResolutionWarning::<__S__>>() },
            { .register_type::<ResolvedActionIntent::<__S__>>() },
            { .register_type::<ChunkOwnerId::<__S__>>() },
        );
    }
}
