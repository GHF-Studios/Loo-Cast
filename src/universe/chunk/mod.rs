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

pub enum ChunkOperationRequest {
    Register {
        id: ChunkID,
    },
    Unregister {
        id: ChunkID,
    },
    LoadMetadata {
        id: ChunkID,
        metadata: ChunkMetadata,
    },
    UnloadMetadata {
        id: ChunkID,
    },
    LoadData {
        id: ChunkID,
        data: ChunkData,
    },
    UnloadData {
        id: ChunkID,
    },
    Spawn {
        id: ChunkID,
    },
    Despawn {
        id: ChunkID,
    },
}

// Structs
pub struct ChunkPlugin;

#[derive(Component)]
pub struct ChunkViewer {
    previously_viewed_chunk_positions: Vec<LocalChunkPos>,
    currently_viewed_chunk_positions: Vec<LocalChunkPos>,
    newly_viewed_chunk_positions: Vec<LocalChunkPos>,
}

#[derive(Component)]
pub struct ChunkECSEntity {
    pub chunk: Arc<Mutex<Chunk>>,
}

#[derive(Resource)]
pub struct ChunkManager {
    registered_root_chunks: Arc<Mutex<HashMap<LocalChunkPos, Arc<Mutex<Chunk>>>>>,
    register_requests: Arc<Mutex<Vec<ChunkID>>>,
    unregister_requests: Arc<Mutex<Vec<ChunkID>>>,
    load_metadata_requests: Arc<Mutex<Vec<(ChunkID, ChunkMetadata)>>>,
    unload_metadata_requests: Arc<Mutex<Vec<ChunkID>>>,
    load_data_requests: Arc<Mutex<Vec<(ChunkID, ChunkData)>>>,
    unload_data_requests: Arc<Mutex<Vec<ChunkID>>>,
    spawn_requests: Arc<Mutex<Vec<ChunkID>>>,
    despawn_requests: Arc<Mutex<Vec<ChunkID>>>,
    operation_requests: Arc<
        Mutex<
            Vec<(
                ChunkOperationRequest,
                Box<dyn Fn() + Send>,
                Box<dyn Fn(ChunkOperationRequest, String) + Send>,
            )>,
        >,
    >,
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
                    ChunkManager::handle_register_requests,
                    ChunkManager::handle_unregister_requests,
                    ChunkManager::handle_load_metadata_requests,
                    ChunkManager::handle_unload_metadata_requests,
                    ChunkManager::handle_load_data_requests,
                    ChunkManager::handle_unload_data_requests,
                    ChunkManager::handle_spawn_requests,
                    ChunkManager::handle_despawn_requests,
                    Chunk::render_system,
                    ChunkViewer::detect_chunks_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), ChunkManager::terminate);
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

        let mut parent_chunk_id_base10x10;
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
        let mut parent_chunk_id;
        let mut chunk_metadata;
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
        let mut chunk_metadata;
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
            .push(entity_id.get_local_id());
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

            let mut chunk_metadata = match *chunk {
                Chunk::Registered { .. } => {
                    continue;
                }
                Chunk::MetadataLoaded { metadata, .. } => metadata,
                Chunk::DataLoaded { metadata, .. } => metadata,
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
            register_requests: Arc::new(Mutex::new(Vec::new())),
            unregister_requests: Arc::new(Mutex::new(Vec::new())),
            load_metadata_requests: Arc::new(Mutex::new(Vec::new())),
            unload_metadata_requests: Arc::new(Mutex::new(Vec::new())),
            load_data_requests: Arc::new(Mutex::new(Vec::new())),
            unload_data_requests: Arc::new(Mutex::new(Vec::new())),
            spawn_requests: Arc::new(Mutex::new(Vec::new())),
            despawn_requests: Arc::new(Mutex::new(Vec::new())),
            operation_requests: Arc::new(Mutex::new(Vec::new())),
        };

        commands.insert_resource(chunk_manager);
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<Self>();
    }

    pub fn get_registered_chunk(&self, id: ChunkID) -> Option<Arc<Mutex<Chunk>>> {
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
            let current_chunk = registered_chunk.lock().ok()?;
            let current_chunk_metadata = match *current_chunk {
                Chunk::Registered { .. } => return None,
                Chunk::MetadataLoaded { metadata, .. } | Chunk::DataLoaded { metadata, .. } => {
                    metadata
                }
            };
            registered_chunk = current_chunk_metadata.child_chunks?.get(&local_id)?.clone();
        }

        Some(registered_chunk)
    }

    pub fn is_chunk_registered(&self, id: ChunkID) -> bool {
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
            let current_chunk = match registered_chunk.lock() {
                Ok(current_chunk) => current_chunk,
                Err(_) => return false,
            };
            let current_chunk_metadata = match *current_chunk {
                Chunk::Registered { .. } => return false,
                Chunk::MetadataLoaded { metadata, .. } | Chunk::DataLoaded { metadata, .. } => {
                    metadata
                }
            };
            let current_chunk_child_chunks = match current_chunk_metadata.child_chunks {
                Some(current_chunk_child_chunks) => current_chunk_child_chunks,
                None => return false,
            };
            registered_chunk = match current_chunk_child_chunks.get(&local_id) {
                Some(registered_chunk) => registered_chunk.clone(),
                None => return false,
            };
        }

        true
    }

    pub fn request_operation(
        &mut self,
        request: ChunkOperationRequest,
        success_callback: Box<dyn Fn() + Send>,
        failure_callback: Box<dyn Fn(ChunkOperationRequest, String) + Send>,
    ) -> Result<(), String> {
        let mut requests = match self.operation_requests.lock() {
            Ok(requests) => requests,
            Err(_) => {
                return Err(
                    "Failed to request chunk operation: Requests mutex poisoned.".to_string(),
                )
            }
        };
        requests.push((request, success_callback, failure_callback));
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
            let (request, success_callback, failure_callback) = operation_request;

            match request {
                ChunkOperationRequest::Register { id } => {
                    match Self::register_chunk(&mut chunk_manager, id.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::Unregister { id } => {
                    match Self::unregister_chunk(&mut chunk_manager, id.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::LoadMetadata { id, metadata } => {
                    match Self::load_chunk_metadata(
                        &mut chunk_manager,
                        id.clone(),
                        metadata.clone(),
                    ) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::UnloadMetadata { id } => {
                    match Self::unload_chunk_metadata(&mut chunk_manager, id.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::LoadData { id, data } => {
                    match Self::load_chunk_data(&mut chunk_manager, id.clone(), data.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::UnloadData { id } => {
                    match Self::unload_chunk_data(&mut chunk_manager, id.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::Spawn { id } => {
                    match Self::spawn_chunk(&mut commands, &mut chunk_manager, id.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
                ChunkOperationRequest::Despawn { id } => {
                    match Self::despawn_chunk(&mut commands, &mut chunk_manager, id.clone()) {
                        Ok(_) => {
                            success_callback();
                        }
                        Err(error) => {
                            failure_callback(request, error);
                        }
                    }
                }
            }
        }
    }

    fn register_chunk(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<(), String> {
        if chunk_manager.get_registered_chunk(chunk_id).is_some() {
            return Err("Failed to register chunk: Chunk is already registered.".to_string());
        }

        if chunk_id.get_scale_index() == 0 {
            let mut registered_root_chunks = match chunk_manager.registered_root_chunks.lock() {
                Ok(registered_root_chunks) => registered_root_chunks,
                Err(_) => {
                    return Err(
                        "Failed to register chunk: Registered root chunks mutex is poisoned."
                            .to_string(),
                    )
                }
            };

            let local_chunk_pos = match chunk_id.compute_local_pos() {
                Ok(local_chunk_pos) => local_chunk_pos,
                Err(_) => {
                    return Err(
                        "Failed to register chunk: Failed to compute local chunk position."
                            .to_string(),
                    );
                }
            };

            let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id)));

            registered_root_chunks.insert(local_chunk_pos, chunk);

            return Ok(());
        }

        let mut parent_id_base10x10 = chunk_id.get_global_id_base10x10().clone();
        parent_id_base10x10.pop();
        let parent_id = match ChunkID::try_from(parent_id_base10x10) {
            Ok(parent_id) => parent_id,
            Err(_) => {
                return Err(
                    "Failed to register chunk: Failed to compute parent chunk id.".to_string(),
                );
            }
        };

        let parent_chunk = match chunk_manager.get_registered_chunk(parent_id) {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err("Failed to register chunk: Parent chunk is not registered.".to_string());
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err("Failed to register chunk: Parent chunk mutex is poisoned.".to_string());
            }
        };

        let parent_chunk_metadata = match Self::get_metadata(&parent_chunk) {
            Ok(parent_chunk_metadata) => parent_chunk_metadata,
            Err(_) => {
                return Err(
                    "Failed to register chunk: Parent chunk metadata is not loaded.".to_string(),
                );
            }
        };

        let mut parent_chunk_child_chunks = match parent_chunk_metadata.child_chunks {
            Some(parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err(
                    "Failed to register chunk: Parent chunk is not allowed to have child chunks."
                        .to_string(),
                );
            }
        };

        let local_chunk_pos = match chunk_id.compute_local_pos() {
            Ok(local_chunk_pos) => local_chunk_pos,
            Err(_) => {
                return Err(
                    "Failed to register chunk: Failed to compute local chunk position.".to_string(),
                );
            }
        };

        if parent_chunk_child_chunks.contains_key(&local_chunk_pos) {
            return Err("Failed to register chunk: Chunk is already registered.".to_string());
        }

        let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id)));

        parent_chunk_child_chunks.insert(local_chunk_pos, chunk);

        return Ok(());
    }

    fn unregister_chunk(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<(), String> {
        let chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err(
                    "Failed to unregister chunk: Chunk is already unregistered.".to_string()
                );
            }
        };

        if chunk_id.get_scale_index() == 0 {
            let mut registered_root_chunks = match chunk_manager.registered_root_chunks.lock() {
                Ok(registered_root_chunks) => registered_root_chunks,
                Err(_) => {
                    return Err(
                        "Failed to unregister chunk: Registered root chunks mutex is poisoned."
                            .to_string(),
                    )
                }
            };

            let local_chunk_pos = match chunk_id.compute_local_pos() {
                Ok(local_chunk_pos) => local_chunk_pos,
                Err(_) => {
                    return Err(
                        "Failed to unregister chunk: Failed to compute local chunk position."
                            .to_string(),
                    );
                }
            };

            match registered_root_chunks.remove(&local_chunk_pos) {
                Some(_) => {}
                None => {
                    return Err(
                        "Failed to unregister chunk: Chunk is already unregistered.".to_string()
                    );
                }
            };

            return Ok(());
        }

        let chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to unregister chunk: Chunk mutex is poisoned.".to_string());
            }
        };

        let parent_chunk_id = match chunk_id.compute_parent_id() {
            Ok(parent_chunk_id) => parent_chunk_id,
            Err(_) => {
                return Err(
                    "Failed to unregister chunk: Failed to compute parent chunk id.".to_string(),
                );
            }
        };

        let parent_chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err(
                    "Failed to unregister chunk: Parent chunk is not registered.".to_string(),
                );
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err(
                    "Failed to unregister chunk: Parent chunk mutex is poisoned.".to_string(),
                );
            }
        };

        let parent_chunk_metadata = match Self::get_metadata(&parent_chunk) {
            Ok(parent_chunk_metadata) => parent_chunk_metadata,
            Err(_) => {
                return Err(
                    "Failed to unregister chunk: Parent chunk metadata is not loaded.".to_string(),
                );
            }
        };

        let mut parent_chunk_child_chunks = match parent_chunk_metadata.child_chunks {
            Some(parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err(
                    "Failed to unregister chunk: Parent chunk is not allowed to have child chunks."
                        .to_string(),
                );
            }
        };

        let local_chunk_pos = match chunk_id.compute_local_pos() {
            Ok(local_chunk_pos) => local_chunk_pos,
            Err(_) => {
                return Err(
                    "Failed to unregister chunk: Failed to compute local chunk position."
                        .to_string(),
                );
            }
        };

        match parent_chunk_child_chunks.remove(&local_chunk_pos) {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err(
                    "Failed to unregister chunk: Chunk is already unregistered.".to_string()
                );
            }
        };
    }

    fn load_chunk_metadata(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
        chunk_metadata: ChunkMetadata,
    ) -> Result<(), String> {
        let chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err("Failed to load chunk metadata: Chunk is not registered.".to_string());
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to load chunk metadata: Chunk mutex is poisoned.".to_string());
            }
        };

        match *chunk {
            Chunk::Registered { id } => {
                *chunk = Chunk::MetadataLoaded {
                    id,
                    metadata: chunk_metadata,
                };
                return Ok(());
            }
            Chunk::MetadataLoaded { .. } | Chunk::DataLoaded { .. } => {
                return Err(
                    "Failed to load chunk metadata: Chunk metadata is already loaded.".to_string(),
                );
            }
        }
    }

    fn unload_chunk_metadata(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<(), String> {
        let chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err("Failed to unload chunk metadata: Chunk is not registered.".to_string());
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to unload chunk metadata: Chunk mutex is poisoned.".to_string());
            }
        };

        let chunk_metadata = match Self::get_metadata(&chunk) {
            Ok(chunk_metadata) => chunk_metadata,
            Err(_) => {
                return Err(
                    "Failed to unload chunk metadata: Chunk metadata is already unloaded."
                        .to_string(),
                );
            }
        };
        if chunk_metadata
            .child_chunks
            .as_ref()
            .map_or(false, |c| !c.is_empty())
        {
            return Err(
                "Failed to unload chunk metadata: Chunk has registered child chunks.".to_string(),
            );
        }

        match *chunk {
            Chunk::Registered { .. } => {
                return Err(
                    "Failed to unload chunk metadata: Chunk metadata is already unloaded."
                        .to_string(),
                );
            }
            Chunk::MetadataLoaded { id, .. } => {
                *chunk = Chunk::Registered { id };
                return Ok(());
            }
            Chunk::DataLoaded { .. } => {
                return Err(
                    "Failed to unload chunk metadata: Chunk data has to be unloaded first."
                        .to_string(),
                );
            }
        }
    }

    fn load_chunk_data(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
        chunk_data: ChunkData,
    ) -> Result<(), String> {
        let chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err("Failed to load chunk data: Chunk is not registered.".to_string());
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to load chunk data: Chunk mutex is poisoned.".to_string());
            }
        };

        match *chunk {
            Chunk::Registered { .. } => {
                return Err("Failed to load chunk data: Chunk metadata is not loaded.".to_string());
            }
            Chunk::MetadataLoaded { id, metadata } => {
                *chunk = Chunk::DataLoaded {
                    id,
                    metadata,
                    data: chunk_data,
                };
                return Ok(());
            }
            Chunk::DataLoaded { .. } => {
                return Err("Failed to load chunk data: Chunk data is already loaded.".to_string());
            }
        }
    }

    fn unload_chunk_data(
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<(), String> {
        let chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err("Failed to unload chunk data: Chunk is not registered.".to_string());
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to unload chunk data: Chunk mutex is poisoned.".to_string());
            }
        };

        let chunk_data = match Self::get_data(&chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err(
                    "Failed to unload chunk data: Chunk data is already unloaded.".to_string(),
                );
            }
        };
        if chunk_data.run_state != ChunkRunState::Despawned {
            return Err("Failed to unload chunk data: Chunk is still spawned.".to_string());
        }

        match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(
                    "Failed to unload chunk data: Chunk data is already unloaded.".to_string(),
                );
            }
            Chunk::DataLoaded { id, metadata, .. } => {
                *chunk = Chunk::MetadataLoaded { id, metadata };
                return Ok(());
            }
        }
    }

    fn spawn_chunk(
        commands: &mut Commands,
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<(), String> {
        let chunk_mutex = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk_mutex) => chunk_mutex,
            None => {
                return Err("Failed to spawn chunk: Chunk is not registered.".to_string());
            }
        };
        let mut chunk = match chunk_mutex.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to spawn chunk: Chunk mutex is poisoned.".to_string());
            }
        };

        let mut chunk_data = match Self::get_data_mut(&mut chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err("Failed to spawn chunk: Chunk data is not loaded.".to_string());
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
                return Ok(());
            }
            ChunkRunState::Spawned { .. } => {
                return Err("Failed to spawn chunk: Chunk is already spawned.".to_string());
            }
        }
    }

    fn despawn_chunk(
        commands: &mut Commands,
        chunk_manager: &mut ResMut<ChunkManager>,
        chunk_id: ChunkID,
    ) -> Result<(), String> {
        let chunk = match chunk_manager.get_registered_chunk(chunk_id) {
            Some(chunk) => chunk,
            None => {
                return Err("Failed to despawn chunk: Chunk is not registered.".to_string());
            }
        };
        let mut chunk = match chunk.lock() {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err("Failed to despawn chunk: Chunk mutex is poisoned.".to_string());
            }
        };

        let mut chunk_data = match Self::get_data_mut(&mut chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err("Failed to despawn chunk: Chunk data is not loaded.".to_string());
            }
        };

        match chunk_data.run_state {
            ChunkRunState::Despawned => {
                return Err("Failed to despawn chunk: Chunk is already despawned.".to_string());
            }
            ChunkRunState::Spawned { ecs_entity } => {
                commands.entity(ecs_entity).despawn();
                chunk_data.run_state = ChunkRunState::Despawned;
                return Ok(());
            }
        }
    }
}

impl ChunkViewer {
    pub fn new() -> ChunkViewer {
        Self {
            previously_viewed_chunk_positions: Vec::new(),
            currently_viewed_chunk_positions: Vec::new(),
            newly_viewed_chunk_positions: Vec::new(),
        }
    }

    fn detect_chunks_system(
        mut chunk_viewer_query: Query<(&mut ChunkViewer, &Transform)>,
        chunk_manager: ResMut<ChunkManager>,
    ) {
        Self::gather_chunk_positions(&mut chunk_viewer_query);
        Self::process_chunk_positions(&mut chunk_viewer_query, chunk_manager);
    }

    fn gather_chunk_positions(chunk_viewer_query: &mut Query<(&mut ChunkViewer, &Transform)>) {
        for (mut chunk_viewer, chunk_viewer_transform) in chunk_viewer_query.iter_mut() {
            if chunk_viewer.previously_viewed_chunk_positions.len() > 0 {
                panic!("Chunk viewer's previously viewed chunk positions are not empty");
            }
            if chunk_viewer.newly_viewed_chunk_positions.len() > 0 {
                panic!("Chunk viewer's newly viewed chunk positions are not empty");
            }

            let chunk_viewer_local_entity_pos: LocalEntityPos =
                chunk_viewer_transform.translation.into();
            let chunk_viewer_local_chunk_position: LocalChunkPos =
                chunk_viewer_local_entity_pos.into();
            let detected_chunk_positions =
                Self::get_chunks_in_range(&chunk_viewer_local_chunk_position);
            let currently_viewed_chunk_positions =
                chunk_viewer.currently_viewed_chunk_positions.clone();

            for currently_viewed_chunk_position in currently_viewed_chunk_positions {
                if !detected_chunk_positions.contains(&currently_viewed_chunk_position) {
                    chunk_viewer
                        .previously_viewed_chunk_positions
                        .push(currently_viewed_chunk_position);
                }
            }

            for detected_chunk_position in &detected_chunk_positions {
                if !chunk_viewer
                    .currently_viewed_chunk_positions
                    .contains(detected_chunk_position)
                {
                    chunk_viewer
                        .newly_viewed_chunk_positions
                        .push(*detected_chunk_position);
                }
            }
        }
    }

    fn process_chunk_positions(
        chunk_viewer_query: &mut Query<(&mut ChunkViewer, &Transform)>,
        mut chunk_manager: ResMut<ChunkManager>,
    ) {
        for (mut chunk_viewer, _) in chunk_viewer_query.iter_mut() {
            // Unload chunks that have exited the view
            let old_chunk_positions = chunk_viewer.previously_viewed_chunk_positions.clone();

            for chunk_pos in &old_chunk_positions {
                chunk_manager.request_unload(*chunk_pos);
            }

            chunk_viewer
                .currently_viewed_chunk_positions
                .retain(|chunk_pos| !old_chunk_positions.contains(chunk_pos));

            chunk_viewer.previously_viewed_chunk_positions.clear();

            // Load chunks that have entered the view
            let mut new_chunks_positions = chunk_viewer.newly_viewed_chunk_positions.clone();

            for chunk_pos in &new_chunks_positions {
                chunk_manager.request_load(*chunk_pos);
            }

            chunk_viewer
                .currently_viewed_chunk_positions
                .append(&mut new_chunks_positions);

            chunk_viewer.newly_viewed_chunk_positions.clear();
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
