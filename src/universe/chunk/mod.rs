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
use crate::universe::*;
use crate::universe::entity::pos::*;
use crate::game::SimulationState;
use crate::AppState;

// External imports
use bevy::prelude::*;
use std::collections::HashMap;
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
    FailedToComputeLocalChunkPosition,
    FailedToComputeParentChunkID,
    FailedToGetParentChunk,
}

#[derive(Debug)]
pub enum UnregisterChunkError {
    RegisteredRootChunksMutexPoisoned,
    ParentChunkMutexPoisoned,
    ChunkMutexPoisoned,
    ChunkDataStillLoaded,
    ChunkMetadataStillLoaded,
    ParentChunkNotRegistered,
    ParentChunkMetadataNotLoaded,
    ParentChunkNotAllowedToHaveChildChunks,
    FailedToComputeLocalChunkPosition,
    FailedToComputeParentChunkID,
    FailedToGetChunk,
    FailedToGetParentChunk,
    ChunkAlreadyUnregistered,
}

#[derive(Debug)]
pub enum LoadChunkMetadataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkMetadataAlreadyLoaded,
    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum UnloadChunkMetadataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkHasRegisteredChildChunks,
    ChunkDataStillLoaded,
    ChunkMetadataAlreadyUnloaded,
    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum LoadChunkDataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ParentChunkMutexPoisoned,
    ChunkMetadataNotLoaded,
    ChunkDataAlreadyLoaded,
    ParentChunkDataNotLoaded,
    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum UnloadChunkDataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkStillSpawned,
    ChunkDataAlreadyUnloaded,
    ChildChunksStillRegistered,
    FailedToGetChunk,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum SpawnChunkError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ParentChunkMutexPoisoned,
    ChunkDataNotLoaded,
    ChunkAlreadySpawned,
    ParentChunkNotSpawned,
    FailedToGetChunk,
}

#[derive(Debug)]
pub enum DespawnChunkError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
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
pub struct ChunkViewer {
    previously_viewed_local_chunk_positions: Vec<LocalChunkPos>,
    currently_viewed_local_chunk_positions: Vec<LocalChunkPos>,
    newly_viewed_local_chunk_positions: Vec<LocalChunkPos>,
}

#[derive(Component)]
pub struct ChunkBevyEntity {
    pub chunk: Arc<Mutex<Chunk>>,
}

// Implementations
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Update Systems
            .add_systems(
                Update,
                (
                    Chunk::debug_render_system,
                    ChunkViewer::detect_local_chunks_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
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

    fn debug_render_system(mut gizmos: Gizmos, chunk_ecs_entity_query: Query<&ChunkBevyEntity>) {
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

impl ChunkViewer {
    pub fn new() -> ChunkViewer {
        Self {
            previously_viewed_local_chunk_positions: Vec::new(),
            currently_viewed_local_chunk_positions: Vec::new(),
            newly_viewed_local_chunk_positions: Vec::new(),
        }
    }

    fn detect_local_chunks_system(
        mut chunk_viewer_query: Query<(&mut ChunkViewer, &Transform)>,
        universe_manager: ResMut<UniverseManager>,
    ) {
        Self::gather_local_chunk_positions(&mut chunk_viewer_query);
        Self::process_local_chunk_positions(&mut chunk_viewer_query, universe_manager);
    }

    fn gather_local_chunk_positions(
        chunk_viewer_query: &mut Query<(&mut ChunkViewer, &Transform)>,
    ) {
        for (mut chunk_viewer, chunk_viewer_transform) in chunk_viewer_query.iter_mut() {
            if chunk_viewer.previously_viewed_local_chunk_positions.len() > 0 {
                panic!("Chunk viewer's previously viewed chunk positions are not empty");
            }
            if chunk_viewer.newly_viewed_local_chunk_positions.len() > 0 {
                panic!("Chunk viewer's newly viewed chunk positions are not empty");
            }

            let chunk_viewer_local_entity_pos: LocalEntityPos =
                chunk_viewer_transform.translation.into();
            let chunk_viewer_local_chunk_position: LocalChunkPos =
                chunk_viewer_local_entity_pos.into();
            let detected_chunk_positions =
                Self::get_chunks_in_range(&chunk_viewer_local_chunk_position);
            let currently_viewed_chunk_positions =
                chunk_viewer.currently_viewed_local_chunk_positions.clone();

            for currently_viewed_chunk_position in currently_viewed_chunk_positions {
                if !detected_chunk_positions.contains(&currently_viewed_chunk_position) {
                    chunk_viewer
                        .previously_viewed_local_chunk_positions
                        .push(currently_viewed_chunk_position);
                }
            }

            for detected_chunk_position in &detected_chunk_positions {
                if !chunk_viewer
                    .currently_viewed_local_chunk_positions
                    .contains(detected_chunk_position)
                {
                    chunk_viewer
                        .newly_viewed_local_chunk_positions
                        .push(detected_chunk_position.clone());
                }
            }
        }
    }

    fn process_local_chunk_positions(
        chunk_viewer_query: &mut Query<(&mut ChunkViewer, &Transform)>,
        mut universe_manager: ResMut<UniverseManager>,
    ) {
        for (mut chunk_viewer, _) in chunk_viewer_query.iter_mut() {
            // Unload chunks that have exited the view
            let old_local_chunk_positions =
                chunk_viewer.previously_viewed_local_chunk_positions.clone();

            for old_local_chunk_pos in &old_local_chunk_positions {
                let old_local_chunk_pos = old_local_chunk_pos.clone();
                let old_local_chunk_pos_base10x10: (u8, u8) = old_local_chunk_pos.into();
                let old_chunk_id = match ChunkID::try_from(old_local_chunk_pos_base10x10) {
                    Ok(old_chunk_id) => old_chunk_id,
                    Err(_) => {
                        continue;
                    }
                };

                match universe_manager.send_chunk_operation_request(ChunkOperationRequest {
                    operations: vec![
                        ChunkOperation::Despawn {
                            id: old_chunk_id.clone(),
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                        ChunkOperation::UnloadData {
                            id: old_chunk_id.clone(),
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                        ChunkOperation::UnloadMetadata {
                            id: old_chunk_id.clone(),
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                        ChunkOperation::Despawn {
                            id: old_chunk_id,
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                    ],
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
            }

            chunk_viewer
                .currently_viewed_local_chunk_positions
                .retain(|chunk_pos| !old_local_chunk_positions.contains(chunk_pos));

            chunk_viewer.previously_viewed_local_chunk_positions.clear();

            // Load chunks that have entered the view
            let mut new_local_chunk_positions =
                chunk_viewer.newly_viewed_local_chunk_positions.clone();

            for new_local_chunk_pos in &new_local_chunk_positions {
                let new_local_chunk_pos = new_local_chunk_pos.clone();
                let new_local_chunk_pos_base10x10: (u8, u8) = new_local_chunk_pos.clone().into();
                let new_chunk_id = match ChunkID::try_from(new_local_chunk_pos_base10x10) {
                    Ok(new_chunk_id) => new_chunk_id,
                    Err(_) => {
                        continue;
                    }
                };

                let new_chunk_metadata = match ChunkMetadata::new(None, new_local_chunk_pos) {
                    Ok(new_chunk_metadata) => new_chunk_metadata,
                    Err(_) => {
                        continue;
                    }
                };

                let new_chunk_data = ChunkData::new();

                match universe_manager.send_chunk_operation_request(ChunkOperationRequest {
                    operations: vec![
                        ChunkOperation::Register {
                            id: new_chunk_id.clone(),
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                        ChunkOperation::LoadMetadata {
                            id: new_chunk_id.clone(),
                            metadata: new_chunk_metadata,
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _, _| {}),
                        },
                        ChunkOperation::LoadData {
                            id: new_chunk_id.clone(),
                            data: new_chunk_data,
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _, _| {}),
                        },
                        ChunkOperation::Spawn {
                            id: new_chunk_id,
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                    ],
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
            }

            chunk_viewer
                .currently_viewed_local_chunk_positions
                .append(&mut new_local_chunk_positions);

            chunk_viewer.newly_viewed_local_chunk_positions.clear();
        }
    }

    fn get_chunks_in_range(center: &LocalChunkPos) -> Vec<LocalChunkPos> {
        let mut chunks = Vec::new();
        let r = VIEW_RADIUS as i8;
        for x in (center.x - r)..=(center.x + r) {
            for y in (center.y - r)..=(center.y + r) {
                chunks.push(LocalChunkPos::new(x, y));
            }
        }
        chunks
    }
}

// Module Functions
