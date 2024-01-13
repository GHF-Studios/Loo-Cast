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
use crate::system::game::SimulationState;
use crate::system::universe::entity::pos::*;
use crate::system::universe::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables
pub const CHUNK_SIZE: u16 = 128;
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

pub enum ChunkOperation {
    RegisterRoot {
        local_chunk_id: LocalChunkID,
        success_callback: Box<dyn FnOnce(RegisterRootChunkSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(RegisterRootChunkError) + Send>,
    },
    Register {
        parent_chunk: Arc<Mutex<Chunk>>,
        local_chunk_id: LocalChunkID,
        success_callback: Box<dyn FnOnce(RegisterChunkSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(RegisterChunkError) + Send>,
    },
    UnregisterRoot {
        chunk: Arc<Mutex<Chunk>>,
        success_callback: Box<dyn FnOnce(UnregisterRootChunkSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnregisterRootChunkError) + Send>,
    },
    Unregister {
        parent_chunk: Arc<Mutex<Chunk>>,
        chunk: Arc<Mutex<Chunk>>,
        success_callback: Box<dyn FnOnce(UnregisterChunkSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnregisterChunkError) + Send>,
    },
    LoadMetadata {
        chunk: Arc<Mutex<Chunk>>,
        metadata: ChunkMetadata,
        success_callback: Box<dyn FnOnce(LoadChunkMetadataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(LoadChunkMetadataError) + Send>,
    },
    UnloadMetadata {
        chunk: Arc<Mutex<Chunk>>,
        success_callback: Box<dyn FnOnce(UnloadChunkMetadataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnloadChunkMetadataError) + Send>,
    },
    LoadData {
        chunk: Arc<Mutex<Chunk>>,
        data: ChunkData,
        success_callback: Box<dyn FnOnce(LoadChunkDataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(LoadChunkDataError) + Send>,
    },
    UnloadData {
        chunk: Arc<Mutex<Chunk>>,
        success_callback: Box<dyn FnOnce(UnloadChunkDataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnloadChunkDataError) + Send>,
    },
    Spawn {
        parent_chunk: Arc<Mutex<Chunk>>,
        chunk: Arc<Mutex<Chunk>>,
        success_callback: Box<dyn FnOnce(SpawnChunkSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(SpawnChunkError) + Send>,
    },
    Despawn {
        chunk: Arc<Mutex<Chunk>>,
        success_callback: Box<dyn FnOnce(DespawnChunkSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(DespawnChunkError) + Send>,
    },
}

#[derive(Debug)]
pub enum RegisterRootChunkError {
    ChunkAlreadyRegistered,
    FailedToCreateChunkID,
}

#[derive(Debug)]
pub enum RegisterChunkError {
    ParentChunkDataNotLoaded,
    ParentChunkNotAllowedToHaveChildChunks,
    ChunkAlreadyRegistered,
    FailedToCreateChunkID,
}

#[derive(Debug)]
pub enum UnregisterRootChunkError {
    ChunkMetadataStillLoaded,
    ChunkDataStillLoaded,
    ChunkAlreadyUnregistered,
}

#[derive(Debug)]
pub enum UnregisterChunkError {
    ChunkMetadataStillLoaded,
    ChunkDataStillLoaded,
    ParentChunkDataNotLoaded,
    ParentChunkNotAllowedToHaveChildChunks,
    ChunkAlreadyUnregistered,
}

#[derive(Debug)]
pub enum LoadChunkMetadataError {
    ChunkMetadataAlreadyLoaded,
}

#[derive(Debug)]
pub enum UnloadChunkMetadataError {
    ChunkMetadataAlreadyUnloaded,
    ChunkDataStillLoaded,
}

#[derive(Debug)]
pub enum LoadChunkDataError {
    ChunkMetadataNotLoaded,
    ChunkDataAlreadyLoaded,
}

#[derive(Debug)]
pub enum UnloadChunkDataError {
    ChunkDataAlreadyUnloaded,
    ChunkStillSpawned,
    ChildChunksStillRegistered,
    EntitiesStillRegistered,
}

#[derive(Debug)]
pub enum SpawnChunkError {
    ParentChunkDataNotLoaded,
    ParentChunkNotSpawned,
    ParentChunkNotAllowedToHaveChildChunks,
    ChunkDataNotLoaded,
    ChunkAlreadySpawned,
    WrongParentChunk,
}

#[derive(Debug)]
pub enum DespawnChunkError {
    ChunkDataNotLoaded,
    ChunkAlreadyDespawned,
    ChildChunkMutexPoisoned,
    ChildChunkStillSpawned,
    RegisteredEntityMutexPoisoned,
    RegisteredEntityStillSpawned,
}

// Structs
pub struct ChunkPlugin;

pub struct ChunkOperationRequest {
    pub(in crate::system::universe) operations: Vec<ChunkOperation>,
}

#[derive(Debug)]
pub struct RegisterRootChunkSuccess;

#[derive(Debug)]
pub struct RegisterChunkSuccess;

#[derive(Debug)]
pub struct UnregisterRootChunkSuccess;

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
    pub(in crate::system::universe) fn new(id: ChunkID, bevy_entity: bevy::ecs::entity::Entity) -> Self {
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

            let absolute_local_chunk_pos = chunk_metadata.absolute_local_chunk_pos;
            let apparent_chunk_pos_shift = chunk_data.apparent_chunk_pos_shift;
            let apparent_local_chunk_pos: ApparentLocalChunkPos = (absolute_local_chunk_pos, apparent_chunk_pos_shift).into();
            
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
