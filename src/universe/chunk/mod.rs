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
use super::entity::{id::*, pos::*};
use crate::game::SimulationState;
use crate::AppState;

// External imports
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables
pub const CHUNK_SIZE: u16 = 64;
pub const VIEW_RADIUS: u16 = 4;

// Types

// Enums
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

pub enum RegisterChunkError {
    RegisteredRootChunksMutexPoisoned,
    ParentChunkMutexPoisoned,
    ParentChunkNotRegistered,
    ParentChunkMetadataNotLoaded,
    ParentChunkNotAllowedToHaveChildChunks,
    ChunkAlreadyRegistered,
    FailedToComputeLocalChunkPosition,
    FailedToComputeParentChunkID,
}

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
    ChunkAlreadyUnregistered,
}

pub enum LoadChunkMetadataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkMetadataAlreadyLoaded,
    FatalUnexpectedError,
}

pub enum UnloadChunkMetadataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkHasRegisteredChildChunks,
    ChunkDataStillLoaded,
    ChunkMetadataAlreadyUnloaded,
    FatalUnexpectedError,
}

pub enum LoadChunkDataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkMetadataNotLoaded,
    ChunkDataAlreadyLoaded,
    FatalUnexpectedError,
}

pub enum UnloadChunkDataError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkStillSpawned,
    ChunkDataAlreadyUnloaded,
    FatalUnexpectedError,
}

pub enum SpawnChunkError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkDataNotLoaded,
    ChunkAlreadySpawned,
}

pub enum DespawnChunkError {
    ChunkNotRegistered,
    ChunkMutexPoisoned,
    ChunkDataNotLoaded,
    ChunkAlreadyDespawned,
}

// Structs
pub struct ChunkPlugin;

pub struct ChunkOperationRequest {
    operations: Vec<ChunkOperation>,
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
pub struct ChunkECSEntity {
    pub chunk: Arc<Mutex<Chunk>>,
}

#[derive(Resource)]
pub struct ChunkManager {
    registered_root_chunks: Arc<Mutex<HashMap<LocalChunkPos, Arc<Mutex<Chunk>>>>>,
    operation_requests: Arc<Mutex<Vec<ChunkOperationRequest>>>,
}

// Implementations
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), ChunkManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (
                    ChunkManager::handle_operation_requests,
                    Chunk::render_system,
                    ChunkViewer::detect_local_chunks_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), ChunkManager::terminate);
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
    fn new(id: ChunkID) -> Self {
        Chunk::Registered { id }
    }

    pub fn create_chunk_id(&mut self, local_chunk_id: u8) -> Result<ChunkID, String> {
        if local_chunk_id > 99 {
            return Err("Invalid local chunk id.".to_string());
        }

        let parent_chunk_id_base10x10;
        match self {
            Chunk::Registered { id, .. } => {
                parent_chunk_id_base10x10 = id.get_global_id_base10x10().clone();
            }
            Chunk::MetadataLoaded { id, .. } => {
                parent_chunk_id_base10x10 = id.get_global_id_base10x10().clone();
            }
            Chunk::DataLoaded { id, .. } => {
                parent_chunk_id_base10x10 = id.get_global_id_base10x10().clone();
            }
        }

        if parent_chunk_id_base10x10.len() == 63 {
            return Err("Cannot create chunk id: Max scale index reached.".to_string());
        }

        let mut chunk_id_base10x10 = parent_chunk_id_base10x10;
        chunk_id_base10x10.append(&mut vec![(local_chunk_id / 10, local_chunk_id % 10)]);

        match ChunkID::try_from(chunk_id_base10x10) {
            Ok(chunk_id) => Ok(chunk_id),
            Err(e) => Err(format!("Generating chunk id failed: {}", e)),
        }
    }

    pub fn generate_entity_id(&mut self) -> Result<EntityID, String> {
        let parent_chunk_id;
        let chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot generate entity id: No metadata is loaded.".to_string());
            }
            Chunk::MetadataLoaded { id, metadata, .. } => {
                parent_chunk_id = id.clone();
                chunk_metadata = metadata;
            }
            Chunk::DataLoaded { id, metadata, .. } => {
                parent_chunk_id = id.clone();
                chunk_metadata = metadata;
            }
        }

        if chunk_metadata.recycled_local_entity_ids.len() != 0 {
            match EntityID::new(
                parent_chunk_id,
                chunk_metadata.recycled_local_entity_ids.pop().unwrap(),
            ) {
                Ok(entity_id) => {
                    return Ok(entity_id);
                }
                Err(e) => Err(format!("Generating a local entity id failed: {}", e)),
            }
        } else {
            if chunk_metadata.current_local_entity_id == u64::MAX {
                return Err("Local entity id space used up.".to_string());
            }

            match EntityID::new(parent_chunk_id, chunk_metadata.current_local_entity_id) {
                Ok(entity_id) => {
                    chunk_metadata.current_local_entity_id += 1;
                    return Ok(entity_id);
                }
                Err(e) => Err(format!("Generating a local entity id failed: {}", e)),
            }
        }
    }

    pub fn recycle_entity_id(&mut self, entity_id: EntityID) -> Result<(), String> {
        let chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot recycle entity id: No metadata is loaded.".to_string());
            }
            Chunk::MetadataLoaded { metadata, .. } => {
                chunk_metadata = metadata;
            }
            Chunk::DataLoaded { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        if chunk_metadata
            .recycled_local_entity_ids
            .contains(&entity_id.get_local_id())
        {
            return Err("Entity id already recycled.".to_string());
        }

        chunk_metadata
            .recycled_local_entity_ids
            .push(entity_id.get_local_id().clone());
        Ok(())
    }

    fn render_system(mut gizmos: Gizmos, chunk_ecs_entity_query: Query<&ChunkECSEntity>) {
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

impl ChunkManager {
    fn initialize(mut commands: Commands) {
        let chunk_manager = Self {
            registered_root_chunks: Arc::new(Mutex::new(HashMap::new())),
            operation_requests: Arc::new(Mutex::new(Vec::new())),
        };

        commands.insert_resource(chunk_manager);
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<Self>();
    }

    pub fn get_registered_chunk(&self, id: &ChunkID) -> Option<Arc<Mutex<Chunk>>> {
        let mut path = id.get_global_id_base10x10().clone();
        if path.is_empty() {
            return None;
        }
        let root_chunk_id = path.remove(0);
        let registered_root_chunks = self.registered_root_chunks.lock().ok()?;
        let mut registered_chunk = registered_root_chunks
            .get(&LocalChunkPos::from(root_chunk_id))?
            .clone();
        drop(registered_root_chunks);

        for &local_id in &path {
            let local_id: LocalChunkPos = local_id.into();
            let next_chunk = {
                let current_chunk = registered_chunk.lock().ok()?;
                let current_chunk_metadata = match *current_chunk {
                    Chunk::Registered { .. } => return None,
                    Chunk::MetadataLoaded { ref metadata, .. }
                    | Chunk::DataLoaded { ref metadata, .. } => metadata,
                };
                let current_chunk_child_chunks = match current_chunk_metadata.child_chunks {
                    Some(ref current_chunk_child_chunks) => current_chunk_child_chunks,
                    None => return None,
                };
                current_chunk_child_chunks.get(&local_id)?.clone()
            };

            registered_chunk = next_chunk;
        }

        Some(registered_chunk)
    }

    pub fn is_chunk_registered(&self, id: &ChunkID) -> bool {
        let mut path = id.get_global_id_base10x10().clone();
        if path.is_empty() {
            return false;
        }
        let root_chunk_id = path.remove(0);
        let registered_root_chunks = match self.registered_root_chunks.lock() {
            Ok(registered_root_chunks) => registered_root_chunks,
            Err(_) => return false,
        };
        let mut registered_chunk =
            match registered_root_chunks.get(&LocalChunkPos::from(root_chunk_id)) {
                Some(registered_chunk) => registered_chunk.clone(),
                None => return false,
            };
        drop(registered_root_chunks);

        for &local_id in &path {
            let local_id: LocalChunkPos = local_id.into();
            let next_chunk = {
                let current_chunk = match registered_chunk.lock() {
                    Ok(current_chunk) => current_chunk,
                    Err(_) => return false,
                };
                let current_chunk_metadata = match *current_chunk {
                    Chunk::Registered { .. } => return false,
                    Chunk::MetadataLoaded { ref metadata, .. }
                    | Chunk::DataLoaded { ref metadata, .. } => metadata,
                };
                let current_chunk_child_chunks = match current_chunk_metadata.child_chunks {
                    Some(ref current_chunk_child_chunks) => current_chunk_child_chunks,
                    None => return false,
                };
                match current_chunk_child_chunks.get(&local_id) {
                    Some(registered_chunk) => registered_chunk.clone(),
                    None => return false,
                }
            };

            registered_chunk = next_chunk;
        }

        true
    }

    pub fn send_operation_request(&mut self, request: ChunkOperationRequest) -> Result<(), String> {
        let mut operation_requests = match self.operation_requests.lock() {
            Ok(operation_requests) => operation_requests,
            Err(_) => {
                return Err(
                    "Failed to request chunk operation: Requests mutex poisoned.".to_string(),
                )
            }
        };
        operation_requests.push(request);
        return Ok(());
    }

    pub fn get_id(chunk: &Chunk) -> &ChunkID {
        return match *chunk {
            Chunk::Registered { ref id, .. }
            | Chunk::MetadataLoaded { ref id, .. }
            | Chunk::DataLoaded { ref id, .. } => id,
        };
    }

    pub fn get_metadata(chunk: &Chunk) -> Result<&ChunkMetadata, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } => {
                return Err("Failed to get chunk metadata: Chunk is not loaded.".to_string())
            }
            Chunk::MetadataLoaded { ref metadata, .. } | Chunk::DataLoaded { ref metadata, .. } => {
                metadata
            }
        });
    }

    pub fn get_metadata_mut(chunk: &mut Chunk) -> Result<&mut ChunkMetadata, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } => {
                return Err("Failed to get chunk metadata: Chunk is not loaded.".to_string())
            }
            Chunk::MetadataLoaded {
                ref mut metadata, ..
            }
            | Chunk::DataLoaded {
                ref mut metadata, ..
            } => metadata,
        });
    }

    pub fn get_data(chunk: &Chunk) -> Result<&ChunkData, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err("Failed to get chunk data: Chunk is not loaded.".to_string())
            }
            Chunk::DataLoaded { ref data, .. } => data,
        });
    }

    pub fn get_data_mut(chunk: &mut Chunk) -> Result<&mut ChunkData, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err("Failed to get chunk data: Chunk is not loaded.".to_string())
            }
            Chunk::DataLoaded { ref mut data, .. } => data,
        });
    }

    pub fn get_load_state(chunk: &Chunk) -> ChunkLoadState {
        return match *chunk {
            Chunk::Registered { .. } => ChunkLoadState::Registered,
            Chunk::MetadataLoaded { .. } => ChunkLoadState::MetadataLoaded,
            Chunk::DataLoaded { .. } => ChunkLoadState::DataLoaded,
        };
    }

    fn handle_operation_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_operation_requests =
            chunk_manager.operation_requests.lock().unwrap_or_else(|_| {
                panic!(
                    "Failed to handle chunk operation requests: Operation requests mutex poisoned."
                )
            });

        let mut operation_requests = Vec::new();
        operation_requests.append(&mut *chunk_manager_operation_requests);

        drop(chunk_manager_operation_requests);

        for operation_request in operation_requests {
            for operation in operation_request.operations {
                match operation {
                    ChunkOperation::Register {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::register_chunk(&mut chunk_manager, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id)) => {
                            failure_callback(error, chunk_id);
                        }
                    },
                    ChunkOperation::Unregister {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::unregister_chunk(&mut chunk_manager, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id)) => {
                            failure_callback(error, chunk_id);
                        }
                    },
                    ChunkOperation::LoadMetadata {
                        id,
                        metadata,
                        success_callback,
                        failure_callback,
                    } => match Self::load_chunk_metadata(&mut chunk_manager, id, metadata) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id, chunk_metadata)) => {
                            failure_callback(error, chunk_id, chunk_metadata);
                        }
                    },
                    ChunkOperation::UnloadMetadata {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::unload_chunk_metadata(&mut chunk_manager, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id)) => {
                            failure_callback(error, chunk_id);
                        }
                    },
                    ChunkOperation::LoadData {
                        id,
                        data,
                        success_callback,
                        failure_callback,
                    } => match Self::load_chunk_data(&mut chunk_manager, id, data) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id, chunk_data)) => {
                            failure_callback(error, chunk_id, chunk_data);
                        }
                    },
                    ChunkOperation::UnloadData {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::unload_chunk_data(&mut chunk_manager, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id)) => {
                            failure_callback(error, chunk_id);
                        }
                    },
                    ChunkOperation::Spawn {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::spawn_chunk(&mut commands, &mut chunk_manager, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id)) => {
                            failure_callback(error, chunk_id);
                        }
                    },
                    ChunkOperation::Despawn {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::despawn_chunk(&mut commands, &mut chunk_manager, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, chunk_id)) => {
                            failure_callback(error, chunk_id);
                        }
                    },
                }
            }
        }
    }

    fn register_chunk(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<RegisterChunkSuccess, (RegisterChunkError, ChunkID)> {
        if chunk_manager.get_registered_chunk(&chunk_id).is_some() {
            return Err((RegisterChunkError::ChunkAlreadyRegistered, chunk_id));
        }

        if chunk_id.get_scale_index().clone() == 0 {
            let mut registered_root_chunks = match chunk_manager.registered_root_chunks.lock() {
                Ok(registered_root_chunks) => registered_root_chunks,
                Err(_) => {
                    return Err((
                        RegisterChunkError::RegisteredRootChunksMutexPoisoned,
                        chunk_id,
                    ));
                }
            };

            let local_chunk_pos = match chunk_id.compute_local_pos() {
                Ok(local_chunk_pos) => local_chunk_pos,
                Err(_) => {
                    return Err((
                        RegisterChunkError::FailedToComputeLocalChunkPosition,
                        chunk_id,
                    ));
                }
            };

            let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id)));

            registered_root_chunks.insert(local_chunk_pos, chunk);

            return Ok(RegisterChunkSuccess);
        }

        let mut parent_id_base10x10 = chunk_id.get_global_id_base10x10().clone();
        parent_id_base10x10.pop();
        let parent_id = match ChunkID::try_from(parent_id_base10x10) {
            Ok(parent_id) => parent_id,
            Err(_) => {
                return Err((RegisterChunkError::FailedToComputeParentChunkID, chunk_id));
            }
        };

        let parent_chunk = match chunk_manager.get_registered_chunk(&parent_id) {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((RegisterChunkError::ParentChunkNotRegistered, chunk_id));
            }
        };
        let mut parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((RegisterChunkError::ParentChunkMutexPoisoned, chunk_id));
            }
        };

        let parent_chunk_metadata = match Self::get_metadata_mut(&mut parent_chunk) {
            Ok(parent_chunk_metadata) => parent_chunk_metadata,
            Err(_) => {
                return Err((RegisterChunkError::ParentChunkMetadataNotLoaded, chunk_id));
            }
        };

        let parent_chunk_child_chunks = match parent_chunk_metadata.child_chunks {
            Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err((
                    RegisterChunkError::ParentChunkNotAllowedToHaveChildChunks,
                    chunk_id,
                ));
            }
        };

        let local_chunk_pos = match chunk_id.compute_local_pos() {
            Ok(local_chunk_pos) => local_chunk_pos,
            Err(_) => {
                return Err((
                    RegisterChunkError::FailedToComputeLocalChunkPosition,
                    chunk_id,
                ));
            }
        };

        if parent_chunk_child_chunks.contains_key(&local_chunk_pos) {
            return Err((RegisterChunkError::ChunkAlreadyRegistered, chunk_id));
        }

        let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id)));

        parent_chunk_child_chunks.insert(local_chunk_pos, chunk);

        return Ok(RegisterChunkSuccess);
    }

    fn unregister_chunk(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<UnregisterChunkSuccess, (UnregisterChunkError, ChunkID)> {
        let chunk = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err((UnregisterChunkError::ChunkAlreadyUnregistered, chunk_id));
            }
        };

        if chunk_id.get_scale_index().clone() == 0 {
            let mut registered_root_chunks = match chunk_manager.registered_root_chunks.lock() {
                Ok(registered_root_chunks) => registered_root_chunks,
                Err(_) => {
                    return Err((
                        UnregisterChunkError::RegisteredRootChunksMutexPoisoned,
                        chunk_id,
                    ));
                }
            };

            let local_chunk_pos = match chunk_id.compute_local_pos() {
                Ok(local_chunk_pos) => local_chunk_pos,
                Err(_) => {
                    return Err((
                        UnregisterChunkError::FailedToComputeLocalChunkPosition,
                        chunk_id,
                    ));
                }
            };

            match registered_root_chunks.remove(&local_chunk_pos) {
                Some(_) => {}
                None => {
                    return Err((UnregisterChunkError::ChunkAlreadyUnregistered, chunk_id));
                }
            };

            return Ok(UnregisterChunkSuccess);
        }

        let chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((UnregisterChunkError::ChunkMutexPoisoned, chunk_id));
            }
        };

        match *chunk {
            Chunk::Registered { .. } => {}
            Chunk::MetadataLoaded { .. } => {
                return Err((UnregisterChunkError::ChunkMetadataStillLoaded, chunk_id));
            }
            Chunk::DataLoaded { .. } => {
                return Err((UnregisterChunkError::ChunkDataStillLoaded, chunk_id));
            }
        }

        let parent_chunk_id = match chunk_id.compute_parent_id() {
            Ok(parent_chunk_id) => parent_chunk_id,
            Err(_) => {
                return Err((UnregisterChunkError::FailedToComputeParentChunkID, chunk_id));
            }
        };

        let parent_chunk = match chunk_manager.get_registered_chunk(&parent_chunk_id) {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((UnregisterChunkError::ParentChunkNotRegistered, chunk_id));
            }
        };
        let mut parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((UnregisterChunkError::ParentChunkMutexPoisoned, chunk_id));
            }
        };

        let parent_chunk_metadata = match Self::get_metadata_mut(&mut parent_chunk) {
            Ok(parent_chunk_metadata) => parent_chunk_metadata,
            Err(_) => {
                return Err((UnregisterChunkError::ParentChunkMetadataNotLoaded, chunk_id));
            }
        };

        let parent_chunk_child_chunks = match parent_chunk_metadata.child_chunks {
            Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err((
                    UnregisterChunkError::ParentChunkNotAllowedToHaveChildChunks,
                    chunk_id,
                ));
            }
        };

        let local_chunk_pos = match chunk_id.compute_local_pos() {
            Ok(local_chunk_pos) => local_chunk_pos,
            Err(_) => {
                return Err((
                    UnregisterChunkError::FailedToComputeLocalChunkPosition,
                    chunk_id,
                ));
            }
        };

        match parent_chunk_child_chunks.remove(&local_chunk_pos) {
            Some(_) => {
                return Ok(UnregisterChunkSuccess);
            }
            None => {
                return Err((UnregisterChunkError::ChunkAlreadyUnregistered, chunk_id));
            }
        };
    }

    fn load_chunk_metadata(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
        chunk_metadata: ChunkMetadata,
    ) -> Result<LoadChunkMetadataSuccess, (LoadChunkMetadataError, ChunkID, ChunkMetadata)> {
        let chunk: Arc<Mutex<Chunk>> = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err((
                    LoadChunkMetadataError::ChunkNotRegistered,
                    chunk_id,
                    chunk_metadata,
                ));
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((
                    LoadChunkMetadataError::ChunkMutexPoisoned,
                    chunk_id,
                    chunk_metadata,
                ));
            }
        };

        match &mut *chunk {
            Chunk::Registered { .. } => {
                let mut stolen_chunk = std::mem::take(&mut *chunk);
                if let Chunk::Registered { id, .. } = &mut stolen_chunk {
                    let stolen_id = std::mem::take(&mut *id);
                    *chunk = Chunk::MetadataLoaded {
                        id: stolen_id,
                        metadata: chunk_metadata,
                    };
                    return Ok(LoadChunkMetadataSuccess);
                } else {
                    return Err((
                        LoadChunkMetadataError::FatalUnexpectedError,
                        chunk_id,
                        chunk_metadata,
                    ));
                }
            }
            Chunk::MetadataLoaded { .. } | Chunk::DataLoaded { .. } => {
                return Err((
                    LoadChunkMetadataError::ChunkMetadataAlreadyLoaded,
                    chunk_id,
                    chunk_metadata,
                ));
            }
        }
    }

    fn unload_chunk_metadata(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<UnloadChunkMetadataSuccess, (UnloadChunkMetadataError, ChunkID)> {
        let chunk = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err((UnloadChunkMetadataError::ChunkNotRegistered, chunk_id));
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((UnloadChunkMetadataError::ChunkMutexPoisoned, chunk_id));
            }
        };

        let chunk_metadata = match Self::get_metadata(&chunk) {
            Ok(chunk_metadata) => chunk_metadata,
            Err(_) => {
                return Err((
                    UnloadChunkMetadataError::ChunkMetadataAlreadyUnloaded,
                    chunk_id,
                ));
            }
        };
        if chunk_metadata
            .child_chunks
            .as_ref()
            .map_or(false, |c| !c.is_empty())
        {
            return Err((
                UnloadChunkMetadataError::ChunkHasRegisteredChildChunks,
                chunk_id,
            ));
        }

        match &mut *chunk {
            Chunk::Registered { .. } => {
                return Err((
                    UnloadChunkMetadataError::ChunkMetadataAlreadyUnloaded,
                    chunk_id,
                ));
            }
            Chunk::MetadataLoaded { id, .. } => {
                let stolen_id = std::mem::take(id);
                *chunk = Chunk::Registered { id: stolen_id };
                return Ok(UnloadChunkMetadataSuccess);
            }
            Chunk::DataLoaded { .. } => {
                return Err((UnloadChunkMetadataError::ChunkDataStillLoaded, chunk_id));
            }
        }
    }

    fn load_chunk_data(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
        chunk_data: ChunkData,
    ) -> Result<LoadChunkDataSuccess, (LoadChunkDataError, ChunkID, ChunkData)> {
        let chunk = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err((LoadChunkDataError::ChunkNotRegistered, chunk_id, chunk_data));
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((LoadChunkDataError::ChunkMutexPoisoned, chunk_id, chunk_data));
            }
        };

        match &mut *chunk {
            Chunk::Registered { .. } => {
                return Err((
                    LoadChunkDataError::ChunkMetadataNotLoaded,
                    chunk_id,
                    chunk_data,
                ));
            }
            Chunk::MetadataLoaded { id, metadata } => {
                let stolen_id = std::mem::take(id);
                let stolen_metadata = std::mem::take(metadata);
                *chunk = Chunk::DataLoaded {
                    id: stolen_id,
                    metadata: stolen_metadata,
                    data: chunk_data,
                };
                return Ok(LoadChunkDataSuccess);
            }
            Chunk::DataLoaded { .. } => {
                return Err((
                    LoadChunkDataError::ChunkDataAlreadyLoaded,
                    chunk_id,
                    chunk_data,
                ));
            }
        }
    }

    fn unload_chunk_data(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<UnloadChunkDataSuccess, (UnloadChunkDataError, ChunkID)> {
        let chunk = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err((UnloadChunkDataError::ChunkNotRegistered, chunk_id));
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((UnloadChunkDataError::ChunkMutexPoisoned, chunk_id));
            }
        };

        let chunk_data = match Self::get_data(&chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err((UnloadChunkDataError::ChunkDataAlreadyUnloaded, chunk_id));
            }
        };
        if chunk_data.run_state != ChunkRunState::Despawned {
            return Err((UnloadChunkDataError::ChunkStillSpawned, chunk_id));
        }

        match &mut *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err((UnloadChunkDataError::ChunkDataAlreadyUnloaded, chunk_id));
            }
            Chunk::DataLoaded { id, metadata, .. } => {
                let stolen_id = std::mem::take(id);
                let stolen_metadata = std::mem::take(metadata);
                *chunk = Chunk::MetadataLoaded {
                    id: stolen_id,
                    metadata: stolen_metadata,
                };
                return Ok(UnloadChunkDataSuccess);
            }
        }
    }

    fn spawn_chunk(
        commands: &mut Commands,
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<SpawnChunkSuccess, (SpawnChunkError, ChunkID)> {
        let chunk_mutex = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk_mutex) => chunk_mutex,
            None => {
                return Err((SpawnChunkError::ChunkNotRegistered, chunk_id));
            }
        };
        let mut chunk = match chunk_mutex.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((SpawnChunkError::ChunkMutexPoisoned, chunk_id));
            }
        };

        let chunk_data = match Self::get_data_mut(&mut chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err((SpawnChunkError::ChunkDataNotLoaded, chunk_id));
            }
        };

        match chunk_data.run_state {
            ChunkRunState::Despawned => {
                chunk_data.run_state = ChunkRunState::Spawned {
                    ecs_entity: commands
                        .spawn(ChunkECSEntity {
                            chunk: chunk_mutex.clone(),
                        })
                        .id(),
                };
                return Ok(SpawnChunkSuccess);
            }
            ChunkRunState::Spawned { .. } => {
                return Err((SpawnChunkError::ChunkAlreadySpawned, chunk_id));
            }
        }
    }

    fn despawn_chunk(
        commands: &mut Commands,
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<DespawnChunkSuccess, (DespawnChunkError, ChunkID)> {
        let chunk = match chunk_manager.get_registered_chunk(&chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err((DespawnChunkError::ChunkNotRegistered, chunk_id));
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((DespawnChunkError::ChunkMutexPoisoned, chunk_id));
            }
        };

        let chunk_data = match Self::get_data_mut(&mut chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err((DespawnChunkError::ChunkDataNotLoaded, chunk_id));
            }
        };

        match chunk_data.run_state {
            ChunkRunState::Despawned => {
                return Err((DespawnChunkError::ChunkAlreadyDespawned, chunk_id));
            }
            ChunkRunState::Spawned { ecs_entity } => {
                commands.entity(ecs_entity).despawn();
                chunk_data.run_state = ChunkRunState::Despawned;
                return Ok(DespawnChunkSuccess);
            }
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
        chunk_manager: ResMut<ChunkManager>,
    ) {
        Self::gather_local_chunk_positions(&mut chunk_viewer_query);
        Self::process_local_chunk_positions(&mut chunk_viewer_query, chunk_manager);
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
        mut chunk_manager: ResMut<ChunkManager>,
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

                match chunk_manager.send_operation_request(ChunkOperationRequest {
                    operations: vec![
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

                match chunk_manager.send_operation_request(ChunkOperationRequest {
                    operations: vec![
                        ChunkOperation::Register {
                            id: new_chunk_id.clone(),
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _| {}),
                        },
                        ChunkOperation::LoadMetadata {
                            id: new_chunk_id,
                            metadata: new_chunk_metadata,
                            success_callback: Box::new(|_| {}),
                            failure_callback: Box::new(|_, _, _| {}),
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
