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
use crate::engine::kernel::game::SimulationState;
use crate::engine::kernel::universe::entity::pos::*;
use crate::engine::kernel::universe::*;
use crate::engine::kernel::AppState;

// External imports
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables
pub const CHUNK_SIZE: u16 = 32;
pub const VIEW_RADIUS: u16 = 2;

// Types

// Enums
#[derive(Debug)]
pub enum Chunk {
    Registered {
        id: ChunkID,
        bevy_entity: Entity
    },
    MetadataLoaded {
        id: ChunkID,
        bevy_entity: bevy::ecs::entity::Entity,
        metadata: ChunkMetadata,
    },
    DataLoaded {
        id: ChunkID,
        bevy_entity: bevy::ecs::entity::Entity,
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
        success_callback: Box<dyn Fn(RegisterChunkSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(RegisterChunkError, ChunkID) + Send>,
    },
    Unregister {
        id: ChunkID,
        success_callback: Box<dyn Fn(UnregisterChunkSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(UnregisterChunkError, ChunkID) + Send>,
    },
    LoadMetadata {
        id: ChunkID,
        metadata: ChunkMetadata,
        success_callback: Box<dyn Fn(LoadChunkMetadataSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(LoadChunkMetadataError, ChunkID, ChunkMetadata) + Send>,
    },
    UnloadMetadata {
        id: ChunkID,
        success_callback: Box<dyn Fn(UnloadChunkMetadataSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(UnloadChunkMetadataError, ChunkID) + Send>,
    },
    LoadData {
        id: ChunkID,
        data: ChunkData,
        success_callback: Box<dyn Fn(LoadChunkDataSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(LoadChunkDataError, ChunkID, ChunkData) + Send>,
    },
    UnloadData {
        id: ChunkID,
        success_callback: Box<dyn Fn(UnloadChunkDataSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(UnloadChunkDataError, ChunkID) + Send>,
    },
    Spawn {
        id: ChunkID,
        success_callback: Box<dyn Fn(SpawnChunkSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(SpawnChunkError, ChunkID) + Send>,
    },
    Despawn {
        id: ChunkID,
        success_callback: Box<dyn Fn(DespawnChunkSuccess, ChunkID) + Send>,
        failure_callback: Box<dyn Fn(DespawnChunkError, ChunkID) + Send>,
    },
}

#[derive(Debug)]
pub enum RegisterChunkError {
    RegisteredRootChunksMutexPoisoned,
    ParentChunkMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkDataNotLoaded,
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
    ParentChunkDataNotLoaded,
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
    ChunkDataAlreadyUnloaded,
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
    EntitiesStillRegistered,

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
    RegisteredEntityMutexPoisoned,

    ChunkNotRegistered,
    ChunkDataNotLoaded,
    ChunkAlreadyDespawned,
    ChildChunksStillSpawned,
    RegisteredEntityStillSpawned,

    FailedToGetChunk,
}

// Structs
pub struct ChunkPlugin;

pub struct ChunkOperationRequest {
    pub(in crate::engine::kernel::universe) operations: Vec<ChunkOperation>,
}

#[derive(Debug)]
pub struct RegisterChunkSuccess;

#[derive(Debug)]
pub struct UnregisterChunkSuccess;

#[derive(Debug)]
pub struct LoadChunkMetadataSuccess;

#[derive(Debug)]
pub struct UnloadChunkMetadataSuccess;

#[derive(Debug)]
pub struct LoadChunkDataSuccess;

#[derive(Debug)]
pub struct UnloadChunkDataSuccess;

#[derive(Debug)]
pub struct SpawnChunkSuccess;

#[derive(Debug)]
pub struct DespawnChunkSuccess;

#[derive(Component)]
pub struct ChunkBevyComponent {
    pub chunk: Arc<Mutex<Chunk>>
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
            bevy_entity: bevy::ecs::entity::Entity::PLACEHOLDER
        }
    }
}

impl Chunk {
    pub(in crate::engine::kernel::universe) fn new(id: ChunkID, bevy_entity: bevy::ecs::entity::Entity) -> Self {
        Chunk::Registered { id, bevy_entity }
    }

    fn debug_render_system(mut gizmos: Gizmos, chunk_ecs_entity_query: Query<&ChunkBevyComponent>) {
        for chunk_ecs_entity in chunk_ecs_entity_query.iter() {
            let chunk = match chunk_ecs_entity.chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    continue;
                }
            };

            let (chunk_metadata, chunk_data) = match *chunk {
                Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                    continue;
                },
                Chunk::DataLoaded { ref metadata, ref data, .. } => (metadata, data),
            };

            let absolute_local_chunk_pos = chunk_metadata.absolute_local_chunk_pos.clone();
            let apparent_chunk_pos_shift = chunk_data.apparent_chunk_pos_shift.clone();
            let apparent_local_chunk_pos: ApparentLocalChunkPos = (absolute_local_chunk_pos.clone(), apparent_chunk_pos_shift).into();
            
            let color = if absolute_local_chunk_pos.x == 0 || absolute_local_chunk_pos.x == 9 || absolute_local_chunk_pos.y == 0 || absolute_local_chunk_pos.y == 9 {
                Color::RED
            } else {
                Color::GREEN
            };
            let gizmo_position: LocalEntityPos = apparent_local_chunk_pos.into();
            let gizmo_position: Vec2 = gizmo_position.into();
            gizmos.rect_2d(
                gizmo_position,
                0.,
                Vec2::splat((CHUNK_SIZE as f32) - 8.0),
                color,
            );
        }
    }
}

// Module Functions
