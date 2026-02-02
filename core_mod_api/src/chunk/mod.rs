pub mod components;
pub mod enums;
pub mod errors;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod workflows;

use crate::bevy::prelude::*;
use components::{Chunk, ChunkActor, ChunkLoader};
use enums::ZoomState;
use errors::{DespawnError, SpawnError};
use resources::{ChunkManager, ChunkRenderExecutorRegistry, ChunkRenderHandles};
use systems::{chunk_detection_system, chunk_management_system, chunk_startup_system, chunk_zoom_cooldown_system};

use crate::{core::run_conditions::run_after_startup_finished, time::run_conditions::run_if_not_paused};

pub(crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
            .insert_resource(ChunkRenderExecutorRegistry::default())
            .add_systems(Startup, chunk_startup_system)
            .add_systems(Update, chunk_zoom_cooldown_system.run_if(run_after_startup_finished.and(run_if_not_paused)))
            .add_systems(
                PostUpdate,
                chunk_detection_system
                    .pipe(chunk_management_system)
                    .run_if(run_after_startup_finished.and(run_if_not_paused)),
            )
            .register_type::<Chunk>()
            .register_type::<ChunkActor>()
            .register_type::<ChunkLoader>()
            .register_type::<ChunkManager>()
            .register_type::<ChunkRenderHandles>()
            .register_type::<SpawnError>()
            .register_type::<DespawnError>()
            .register_type::<ZoomState>();
    }
}
