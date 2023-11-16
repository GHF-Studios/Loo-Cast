// Modules
pub mod data;
pub mod id;
pub mod metadata;
pub mod pos;

// Local imports
use data::*;
use id::*;
use metadata::*;
use pos::*;

// Internal imports
use crate::game::SimulationState;
use crate::universe::entity::pos::*;
use crate::universe::*;
use crate::AppState;

// External imports
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables
pub const CHUNK_SIZE: u16 = 512;
pub const VIEW_RADIUS: u16 = 2;

// Types

// Enums
#[derive(Debug)]
pub enum Chunk {
    Registered {
        id: ChunkID,
    },
    MetadataLoaded {
        id: ChunkID,
        metadata: ChunkMetadata,
    },
    DataLoaded {
        id: ChunkID,
        metadata: ChunkMetadata,
        data: ChunkData,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChunkLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

pub enum ChunkOperation {
    Register {
        id: ChunkID,
        success_callback: Box<dyn Fn(RegisterChunkSuccess) + Send>,
        failure_callback: Box<dyn Fn(RegisterChunkError, ChunkID) + Send>,
    },
    Unregister {
        id: ChunkID,
        success_callback: Box<dyn Fn(UnregisterChunkSuccess) + Send>,
        failure_callback: Box<dyn Fn(UnregisterChunkError, ChunkID) + Send>,
    },
    LoadMetadata {
        id: ChunkID,
        metadata: ChunkMetadata,
        success_callback: Box<dyn Fn(LoadChunkMetadataSuccess) + Send>,
        failure_callback: Box<dyn Fn(LoadChunkMetadataError, ChunkID, ChunkMetadata) + Send>,
    },
    UnloadMetadata {
        id: ChunkID,
        success_callback: Box<dyn Fn(UnloadChunkMetadataSuccess) + Send>,
        failure_callback: Box<dyn Fn(UnloadChunkMetadataError, ChunkID) + Send>,
    },
    LoadData {
        id: ChunkID,
        data: ChunkData,
        success_callback: Box<dyn Fn(LoadChunkDataSuccess) + Send>,
        failure_callback: Box<dyn Fn(LoadChunkDataError, ChunkID, ChunkData) + Send>,
    },
    UnloadData {
        id: ChunkID,
        success_callback: Box<dyn Fn(UnloadChunkDataSuccess) + Send>,
        failure_callback: Box<dyn Fn(UnloadChunkDataError, ChunkID) + Send>,
    },
    Spawn {
        id: ChunkID,
        success_callback: Box<dyn Fn(SpawnChunkSuccess) + Send>,
        failure_callback: Box<dyn Fn(SpawnChunkError, ChunkID) + Send>,
    },
    Despawn {
        id: ChunkID,
        success_callback: Box<dyn Fn(DespawnChunkSuccess) + Send>,
        failure_callback: Box<dyn Fn(DespawnChunkError, ChunkID) + Send>,
    },
}

#[derive(Debug)]
pub enum RegisterChunkError {
    RegisteredRootChunksMutexPoisoned,
    ParentChunkMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkMetadataNotLoaded,
    ParentChunkNotAllowedToHaveChildChunks,
    ChunkAlreadyRegistered,

    FailedToComputeParentChunkID,
    FailedToGetParentChunk,
    FailedToComputeLocalChunkPosition,
}

#[derive(Debug)]
pub enum UnregisterChunkError {
    RegisteredRootChunksMutexPoisoned,
    ParentChunkMutexPoisoned,
    ChunkMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkMetadataNotLoaded,
    ParentChunkNotAllowedToHaveChildChunks,
    ChunkDataStillLoaded,
    ChunkMetadataStillLoaded,
    ChunkAlreadyUnregistered,

    FailedToComputeParentChunkID,
    FailedToGetParentChunk,
    FailedToComputeLocalChunkPosition,
    FailedToGetChunk,
}

#[derive(Debug)]
pub enum LoadChunkMetadataError {
    ChunkMutexPoisoned,

    ChunkNotRegistered,
    ChunkMetadataAlreadyLoaded,

    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum UnloadChunkMetadataError {
    ChunkMutexPoisoned,

    ChunkNotRegistered,
    ChunkHasRegisteredChildChunks,
    ChunkMetadataAlreadyUnloaded,
    ChunkDataStillLoaded,

    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum LoadChunkDataError {
    ChunkMutexPoisoned,
    ParentChunkMutexPoisoned,

    ChunkNotRegistered,
    ChunkMetadataNotLoaded,
    ChunkDataAlreadyLoaded,
    ParentChunkDataNotLoaded,

    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum UnloadChunkDataError {
    ChunkMutexPoisoned,

    ChunkNotRegistered,
    ChildChunksStillRegistered,
    ChunkDataAlreadyUnloaded,
    ChunkStillSpawned,

    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum SpawnChunkError {
    ChunkMutexPoisoned,
    ParentChunkMutexPoisoned,

    ParentChunkNotSpawned,
    ChunkNotRegistered,
    ChunkDataNotLoaded,
    ChunkAlreadySpawned,

    FailedToGetChunk,
}

#[derive(Debug)]
pub enum DespawnChunkError {
    ChunkMutexPoisoned,

    ChunkNotRegistered,
    ChunkDataNotLoaded,
    ChunkAlreadyDespawned,
    ChildChunksStillSpawned,

    FailedToGetChunk,
}

// Structs
pub struct ChunkPlugin;

pub struct ChunkOperationRequest {
    pub(in crate::universe) operations: Vec<ChunkOperation>,
}

pub struct RegisterChunkSuccess;
pub struct UnregisterChunkSuccess;
pub struct LoadChunkMetadataSuccess;
pub struct UnloadChunkMetadataSuccess;
pub struct LoadChunkDataSuccess;
pub struct UnloadChunkDataSuccess;
pub struct SpawnChunkSuccess;
pub struct DespawnChunkSuccess;

#[derive(Component)]
pub struct ChunkBevyComponent {
    pub chunk: Arc<Mutex<Chunk>>,
}

// Implementations
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Update Systems
            .add_systems(
                Update,
                (Chunk::debug_render_system,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl ChunkOperationRequest {
    pub fn new(operations: Vec<ChunkOperation>) -> Self {
        ChunkOperationRequest { operations }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk::Registered {
            id: ChunkID::default(),
        }
    }
}

impl Chunk {
    pub(in crate::universe) fn new(id: ChunkID) -> Self {
        Chunk::Registered { id }
    }

    fn debug_render_system(mut gizmos: Gizmos, chunk_ecs_entity_query: Query<&ChunkBevyComponent>) {
        for chunk_ecs_entity in chunk_ecs_entity_query.iter() {
            let chunk = match chunk_ecs_entity.chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    continue;
                }
            };

            let chunk_metadata = match *chunk {
                Chunk::Registered { .. } => {
                    continue;
                }
                Chunk::MetadataLoaded { ref metadata, .. } => metadata,
                Chunk::DataLoaded { ref metadata, .. } => metadata,
            };

            let gizmo_position: LocalEntityPos =
                chunk_metadata.get_pos().get_local_pos().clone().into();
            let gizmo_position: Vec2 = gizmo_position.into();
            gizmos.rect_2d(
                gizmo_position,
                0.,
                Vec2::splat(CHUNK_SIZE as f32),
                Color::RED,
            );
        }
    }
}

// Module Functions
