// Modules
pub mod id;

// Local imports

// Internal imports
use crate::game::SimulationState;
use crate::universe::chunk::data::*;
use crate::universe::chunk::id::*;
use crate::universe::chunk::metadata::*;
use crate::universe::chunk::pos::*;
use crate::universe::chunk::*;
use crate::universe::entity::data::*;
use crate::universe::entity::id::*;
use crate::universe::entity::metadata::*;
use crate::universe::entity::pos::*;
use crate::universe::entity::*;
use crate::universe::local::*;
use crate::universe::*;
use crate::AppState;

// External imports
use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums

// Structs
pub struct GlobalUniversePlugin;

pub struct GlobalUniverse {
    pub(in crate::universe) registered_root_chunks:
        Arc<Mutex<HashMap<LocalChunkPos, Arc<Mutex<Chunk>>>>>,
    pub(in crate::universe) chunk_operation_requests: Arc<Mutex<Vec<ChunkOperationRequest>>>,
    pub(in crate::universe) entity_operation_requests: Arc<Mutex<Vec<EntityOperationRequest>>>,
}

// Implementations
impl Plugin for GlobalUniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Update Systems
            .add_systems(
                Update,
                (GlobalUniverse::handle_operation_requests,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl GlobalUniverse {
    pub fn generate_entity_id(parent_chunk: &mut Chunk) -> Result<EntityID, String> {
        let (parent_chunk_id, parent_chunk_metadata) = match parent_chunk {
            Chunk::Registered { .. } => {
                return Err(
                    "Generating a local entity id failed: Parent chunk metadata is not loaded."
                        .to_string(),
                );
            }
            Chunk::MetadataLoaded { id, metadata, .. } | Chunk::DataLoaded { id, metadata, .. } => {
                (id.clone(), metadata)
            }
        };

        if parent_chunk_metadata.recycled_local_entity_ids.len() != 0 {
            match EntityID::new(
                parent_chunk_id,
                parent_chunk_metadata
                    .recycled_local_entity_ids
                    .pop()
                    .unwrap(),
            ) {
                Ok(entity_id) => {
                    return Ok(entity_id);
                }
                Err(e) => Err(format!("Generating a local entity id failed: {}", e)),
            }
        } else {
            if parent_chunk_metadata.current_local_entity_id == u64::MAX {
                return Err("Generating a local entity id failed: ID space is used up.".to_string());
            }

            match EntityID::new(
                parent_chunk_id,
                parent_chunk_metadata.current_local_entity_id,
            ) {
                Ok(entity_id) => {
                    parent_chunk_metadata.current_local_entity_id += 1;
                    return Ok(entity_id);
                }
                Err(e) => Err(format!("Generating a local entity id failed: {}", e)),
            }
        }
    }

    pub fn recycle_entity_id(parent_chunk: &mut Chunk, entity_id: EntityID) -> Result<(), String> {
        let (_, chunk_metadata, _) = GlobalUniverse::get_chunk_details_mut(parent_chunk);

        let chunk_metadata = match chunk_metadata {
            Some(chunk_metadata) => chunk_metadata,
            None => {
                return Err(
                    "Recycling a local entity id failed: Parent chunk metadata is not loaded."
                        .to_string(),
                );
            }
        };

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

    pub fn get_registered_chunk(
        &self,
        chunk_id: &ChunkID,
    ) -> Result<Option<Arc<Mutex<Chunk>>>, String> {
        let mut path = chunk_id.get_global_id_base10x10().clone();
        if path.is_empty() {
            return Err("Failed to get registered chunk: Invalid chunk id.".to_string());
        }
        let root_chunk_id = path.remove(0);
        let registered_root_chunks = match self.registered_root_chunks.lock().ok() {
            Some(registered_root_chunks) => registered_root_chunks,
            None => {
                return Err(
                    "Failed to get registered chunk: Registered root chunks mutex poisoned."
                        .to_string(),
                )
            }
        };
        let mut registered_chunk = match registered_root_chunks
            .get(&LocalChunkPos::from(root_chunk_id))
        {
            Some(registered_chunk) => registered_chunk.clone(),
            None => {
                return Err("Failed to get registered chunk: Root chunk not registered.".to_string())
            }
        };
        drop(registered_root_chunks);

        for &local_chunk_id in &path {
            let local_chunk_id: LocalChunkPos = local_chunk_id.into();
            let next_chunk = {
                let current_chunk = match registered_chunk.lock().ok() {
                    Some(current_chunk) => current_chunk,
                    None => {
                        return Err(
                            "Failed to get registered chunk: Current chunk mutex poisoned."
                                .to_string(),
                        )
                    }
                };
                let current_chunk_metadata =
                    match *current_chunk {
                        Chunk::Registered { .. } => return Err(
                            "Failed to get registered chunk: Current chunk metadata not loaded."
                                .to_string(),
                        ),
                        Chunk::MetadataLoaded { ref metadata, .. }
                        | Chunk::DataLoaded { ref metadata, .. } => metadata,
                    };
                let current_chunk_child_chunks = match current_chunk_metadata.child_chunks {
                    Some(ref current_chunk_child_chunks) => current_chunk_child_chunks,
                    None => return Err("Failed to get registered chunk: Current chunk not allowed to have child chunks.".to_string()),
                };
                match current_chunk_child_chunks.get(&local_chunk_id) {
                    Some(registered_chunk) => registered_chunk.clone(),
                    None => return Ok(None),
                }
            };

            registered_chunk = next_chunk;
        }

        Ok(Some(registered_chunk))
    }

    pub fn is_chunk_registered(&self, chunk_id: &ChunkID) -> Result<bool, String> {
        let mut path = chunk_id.get_global_id_base10x10().clone();
        if path.is_empty() {
            return Err("Failed to check if chunk is registered: Invalid chunk id.".to_string());
        }
        let root_chunk_id = path.remove(0);
        let registered_root_chunks = match self.registered_root_chunks.lock() {
            Ok(registered_root_chunks) => registered_root_chunks,
            Err(_) => return Err(
                "Failed to check if chunk is registered: Registered root chunks mutex poisoned."
                    .to_string(),
            ),
        };
        let mut registered_chunk =
            match registered_root_chunks.get(&LocalChunkPos::from(root_chunk_id)) {
                Some(registered_chunk) => registered_chunk.clone(),
                None => {
                    return Err(
                        "Failed to check if chunk is registered: Root chunk not registered."
                            .to_string(),
                    )
                }
            };
        drop(registered_root_chunks);

        for &local_chunk_id in &path {
            let local_chunk_id: LocalChunkPos = local_chunk_id.into();
            let next_chunk = {
                let current_chunk =
                    match registered_chunk.lock() {
                        Ok(current_chunk) => current_chunk,
                        Err(_) => return Err(
                            "Failed to check if chunk is registered: Current chunk mutex poisoned."
                                .to_string(),
                        ),
                    };
                let current_chunk_metadata = match *current_chunk {
                    Chunk::Registered { .. } => return Err("Failed to check if chunk is registered: Current chunk metadata not loaded.".to_string()),
                    Chunk::MetadataLoaded { ref metadata, .. }
                    | Chunk::DataLoaded { ref metadata, .. } => metadata,
                };
                let current_chunk_child_chunks = match current_chunk_metadata.child_chunks {
                    Some(ref current_chunk_child_chunks) => current_chunk_child_chunks,
                    None => return Err("Failed to check if chunk is registered: Current chunk not allowed to have child chunks.".to_string()),
                };
                match current_chunk_child_chunks.get(&local_chunk_id) {
                    Some(registered_chunk) => registered_chunk.clone(),
                    None => return Ok(false),
                }
            };

            registered_chunk = next_chunk;
        }

        Ok(true)
    }

    pub fn get_registered_entity(
        parent_chunk: &Chunk,
        entity_id: &EntityID,
    ) -> Result<Option<Arc<Mutex<entity::Entity>>>, String> {
        let (_, parent_chunk_metadata, _) = Self::get_chunk_details(parent_chunk);

        let parent_chunk_metadata = match parent_chunk_metadata {
            Some(parent_chunk_metadata) => parent_chunk_metadata,
            None => {
                return Err(
                    "Failed to get registered entity: Parent chunk metadata not loaded."
                        .to_string(),
                )
            }
        };

        match parent_chunk_metadata
            .registered_entities
            .get(entity_id.get_local_id())
        {
            Some(registered_entity) => Ok(Some(registered_entity.clone())),
            None => Ok(None),
        }
    }

    pub fn is_entity_registered(
        parent_chunk: &Chunk,
        entity_id: &EntityID,
    ) -> Result<bool, String> {
        let (_, parent_chunk_metadata, _) = Self::get_chunk_details(parent_chunk);

        match parent_chunk_metadata {
            Some(parent_chunk_metadata) => Ok(parent_chunk_metadata
                .registered_entities
                .contains_key(entity_id.get_local_id())),
            None => Err(
                "Failed to check if entity is registered: Parent chunk metadata not loaded."
                    .to_string(),
            ),
        }
    }

    pub fn send_chunk_operation_request(
        &mut self,
        request: ChunkOperationRequest,
    ) -> Result<(), String> {
        let mut chunk_operation_requests =
            match self.chunk_operation_requests.lock() {
                Ok(chunk_operation_requests) => chunk_operation_requests,
                Err(_) => return Err(
                    "Failed to request chunk operation: Chunk operation requests mutex poisoned."
                        .to_string(),
                ),
            };
        chunk_operation_requests.push(request);
        return Ok(());
    }

    pub fn send_entity_operation_request(
        &mut self,
        request: EntityOperationRequest,
    ) -> Result<(), String> {
        let mut entity_operation_requests =
            match self.entity_operation_requests.lock() {
                Ok(entity_operation_requests) => entity_operation_requests,
                Err(_) => return Err(
                    "Failed to request entity operation: Entity operation requests mutex poisoned."
                        .to_string(),
                ),
            };
        entity_operation_requests.push(request);
        return Ok(());
    }

    pub fn get_chunk_load_state(chunk: &Chunk) -> ChunkLoadState {
        return match *chunk {
            Chunk::Registered { .. } => ChunkLoadState::Registered,
            Chunk::MetadataLoaded { .. } => ChunkLoadState::MetadataLoaded,
            Chunk::DataLoaded { .. } => ChunkLoadState::DataLoaded,
        };
    }

    pub fn get_chunk_details(
        chunk: &Chunk,
    ) -> (&ChunkID, Option<&ChunkMetadata>, Option<&ChunkData>) {
        return match *chunk {
            Chunk::Registered { ref id } => (id, None, None),
            Chunk::MetadataLoaded {
                ref id,
                ref metadata,
            } => (id, Some(metadata), None),
            Chunk::DataLoaded {
                ref id,
                ref metadata,
                ref data,
            } => (id, Some(metadata), Some(data)),
        };
    }

    pub fn get_chunk_details_mut(
        chunk: &mut Chunk,
    ) -> (&ChunkID, Option<&mut ChunkMetadata>, Option<&mut ChunkData>) {
        return match *chunk {
            Chunk::Registered { ref id } => (id, None, None),
            Chunk::MetadataLoaded {
                ref id,
                ref mut metadata,
            } => (id, Some(metadata), None),
            Chunk::DataLoaded {
                ref id,
                ref mut metadata,
                ref mut data,
            } => (id, Some(metadata), Some(data)),
        };
    }

    pub fn get_chunk_id(chunk: &Chunk) -> &ChunkID {
        return match *chunk {
            Chunk::Registered { ref id } => id,
            Chunk::MetadataLoaded { ref id, .. } => id,
            Chunk::DataLoaded { ref id, .. } => id,
        };
    }

    pub fn get_chunk_metadata(chunk: &Chunk) -> Result<&ChunkMetadata, String> {
        return match *chunk {
            Chunk::Registered { .. } => Err("Chunk metadata not loaded.".to_string()),
            Chunk::MetadataLoaded { ref metadata, .. } => Ok(metadata),
            Chunk::DataLoaded { ref metadata, .. } => Ok(metadata),
        };
    }

    pub fn get_chunk_metadata_mut(chunk: &mut Chunk) -> Result<&mut ChunkMetadata, String> {
        return match *chunk {
            Chunk::Registered { .. } => Err("Chunk metadata not loaded.".to_string()),
            Chunk::MetadataLoaded {
                ref mut metadata, ..
            } => Ok(metadata),
            Chunk::DataLoaded {
                ref mut metadata, ..
            } => Ok(metadata),
        };
    }

    pub fn get_chunk_data(chunk: &Chunk) -> Result<&ChunkData, String> {
        return match *chunk {
            Chunk::Registered { .. } => Err("Chunk data not loaded.".to_string()),
            Chunk::MetadataLoaded { .. } => Err("Chunk data not loaded.".to_string()),
            Chunk::DataLoaded { ref data, .. } => Ok(data),
        };
    }

    pub fn get_chunk_data_mut(chunk: &mut Chunk) -> Result<&mut ChunkData, String> {
        return match *chunk {
            Chunk::Registered { .. } => Err("Chunk data not loaded.".to_string()),
            Chunk::MetadataLoaded { .. } => Err("Chunk data not loaded.".to_string()),
            Chunk::DataLoaded { ref mut data, .. } => Ok(data),
        };
    }

    pub fn get_entity_load_state(entity: &entity::Entity) -> EntityLoadState {
        return match *entity {
            entity::Entity::Registered { .. } => EntityLoadState::Registered,
            entity::Entity::MetadataLoaded { .. } => EntityLoadState::MetadataLoaded,
            entity::Entity::DataLoaded { .. } => EntityLoadState::DataLoaded,
        };
    }

    pub fn get_entity_details(
        entity: &entity::Entity,
    ) -> (&EntityID, Option<&EntityMetadata>, Option<&EntityData>) {
        return match *entity {
            entity::Entity::Registered { ref id } => (id, None, None),
            entity::Entity::MetadataLoaded {
                ref id,
                ref metadata,
            } => (id, Some(metadata), None),
            entity::Entity::DataLoaded {
                ref id,
                ref metadata,
                ref data,
            } => (id, Some(metadata), Some(data)),
        };
    }

    pub fn get_entity_details_mut(
        entity: &mut entity::Entity,
    ) -> (
        &EntityID,
        Option<&mut EntityMetadata>,
        Option<&mut EntityData>,
    ) {
        return match *entity {
            entity::Entity::Registered { ref id } => (id, None, None),
            entity::Entity::MetadataLoaded {
                ref id,
                ref mut metadata,
            } => (id, Some(metadata), None),
            entity::Entity::DataLoaded {
                ref id,
                ref mut metadata,
                ref mut data,
            } => (id, Some(metadata), Some(data)),
        };
    }

    pub fn get_entity_id(entity: &entity::Entity) -> &EntityID {
        return match *entity {
            entity::Entity::Registered { ref id } => id,
            entity::Entity::MetadataLoaded { ref id, .. } => id,
            entity::Entity::DataLoaded { ref id, .. } => id,
        };
    }

    pub fn get_entity_metadata(entity: &entity::Entity) -> Result<&EntityMetadata, String> {
        return match *entity {
            entity::Entity::Registered { .. } => Err("Entity metadata not loaded.".to_string()),
            entity::Entity::MetadataLoaded { ref metadata, .. } => Ok(metadata),
            entity::Entity::DataLoaded { ref metadata, .. } => Ok(metadata),
        };
    }

    pub fn get_entity_metadata_mut(
        entity: &mut entity::Entity,
    ) -> Result<&mut EntityMetadata, String> {
        return match *entity {
            entity::Entity::Registered { .. } => Err("Entity metadata not loaded.".to_string()),
            entity::Entity::MetadataLoaded {
                ref mut metadata, ..
            } => Ok(metadata),
            entity::Entity::DataLoaded {
                ref mut metadata, ..
            } => Ok(metadata),
        };
    }

    pub fn get_entity_data(entity: &entity::Entity) -> Result<&EntityData, String> {
        return match *entity {
            entity::Entity::Registered { .. } => Err("Entity data not loaded.".to_string()),
            entity::Entity::MetadataLoaded { .. } => Err("Entity data not loaded.".to_string()),
            entity::Entity::DataLoaded { ref data, .. } => Ok(data),
        };
    }

    pub fn get_entity_data_mut(entity: &mut entity::Entity) -> Result<&mut EntityData, String> {
        return match *entity {
            entity::Entity::Registered { .. } => Err("Entity data not loaded.".to_string()),
            entity::Entity::MetadataLoaded { .. } => Err("Entity data not loaded.".to_string()),
            entity::Entity::DataLoaded { ref mut data, .. } => Ok(data),
        };
    }

    fn handle_operation_requests(
        mut commands: Commands,
        mut universe_manager: ResMut<UniverseManager>,
    ) {
        let global_universe = match universe_manager.registered_global_universe {
            Some(ref mut global_universe) => global_universe,
            None => {
                return;
            }
        };
        let mut global_universe = match global_universe.lock() {
            Ok(global_universe) => global_universe,
            Err(_) => {
                return;
            }
        };

        let mut global_universe_chunk_operation_requests =
            global_universe.chunk_operation_requests.lock().unwrap_or_else(|_| {
                panic!(
                    "Failed to handle chunk operation requests: Chunk operation requests mutex poisoned."
                )
            });

        let mut chunk_operation_requests = Vec::new();
        chunk_operation_requests.append(&mut *global_universe_chunk_operation_requests);

        drop(global_universe_chunk_operation_requests);

        for chunk_operation_request in chunk_operation_requests {
            for chunk_operation in chunk_operation_request.operations {
                match chunk_operation {
                    ChunkOperation::Register {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::register_chunk(&mut global_universe, id) {
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
                    } => match Self::unregister_chunk(&mut global_universe, id) {
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
                    } => match Self::load_chunk_metadata(&mut global_universe, id, metadata) {
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
                    } => match Self::unload_chunk_metadata(&mut global_universe, id) {
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
                    } => match Self::load_chunk_data(&mut global_universe, id, data) {
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
                    } => match Self::unload_chunk_data(&mut global_universe, id) {
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
                    } => match Self::spawn_chunk(&mut commands, &mut global_universe, id) {
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
                    } => match Self::despawn_chunk(&mut commands, &mut global_universe, id) {
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

        let mut global_universe_entity_operation_requests =
            global_universe.entity_operation_requests.lock().unwrap_or_else(|_| {
                panic!(
                    "Failed to handle entity operation requests: Entity operation requests mutex poisoned."
                )
            });

        let mut entity_operation_requests = Vec::new();
        entity_operation_requests.append(&mut *global_universe_entity_operation_requests);

        drop(global_universe_entity_operation_requests);

        for entity_operation_request in entity_operation_requests {
            for entity_operation in entity_operation_request.operations {
                match entity_operation {
                    EntityOperation::Register {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::register_entity(&mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }
                    },
                    EntityOperation::Unregister {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::unregister_entity(&mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }
                    },
                    EntityOperation::LoadMetadata {
                        id,
                        metadata,
                        success_callback,
                        failure_callback,
                    } => match Self::load_entity_metadata(&mut global_universe, id, metadata) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id, chunk_metadata)) => {
                            failure_callback(error, entity_id, chunk_metadata);
                        }
                    },
                    EntityOperation::UnloadMetadata {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::unload_entity_metadata(&mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }
                    },
                    EntityOperation::LoadData {
                        id,
                        data,
                        success_callback,
                        failure_callback,
                    } => match Self::load_entity_data(&mut global_universe, id, data) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id, entity_data)) => {
                            failure_callback(error, entity_id, entity_data);
                        }
                    },
                    EntityOperation::UnloadData {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::unload_entity_data(&mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }
                    },
                    EntityOperation::Spawn {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::spawn_entity(&mut commands, &mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }
                    },
                    EntityOperation::Despawn {
                        id,
                        success_callback,
                        failure_callback,
                    } => match Self::despawn_entity(&mut commands, &mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }
                    },
                    EntityOperation::CommandEntity { 
                        id, 
                        command, 
                        success_callback, 
                        failure_callback 
                    } => match Self::command_entity(command, &mut commands, &mut global_universe, id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err((error, entity_id)) => {
                            failure_callback(error, entity_id);
                        }

                    }
                }
            }
        }
    }

    fn register_chunk(
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
    ) -> Result<RegisterChunkSuccess, (RegisterChunkError, ChunkID)> {
        if global_universe.get_registered_chunk(&chunk_id).is_ok() {
            return Err((RegisterChunkError::ChunkAlreadyRegistered, chunk_id));
        }

        if chunk_id.get_scale_index().clone() == 0 {
            let mut registered_root_chunks = match global_universe.registered_root_chunks.lock() {
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

        let parent_chunk = match global_universe.get_registered_chunk(&parent_id) {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((RegisterChunkError::FailedToGetParentChunk, chunk_id));
            }
        };
        let parent_chunk = match parent_chunk {
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

        let parent_chunk_metadata = match Self::get_chunk_metadata_mut(&mut parent_chunk) {
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
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
    ) -> Result<UnregisterChunkSuccess, (UnregisterChunkError, ChunkID)> {
        let chunk = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((UnregisterChunkError::FailedToGetChunk, chunk_id));
            }
        };
        let chunk = match chunk {
            Some(chunk) => chunk,
            None => {
                return Err((UnregisterChunkError::ChunkAlreadyUnregistered, chunk_id));
            }
        };

        if chunk_id.get_scale_index().clone() == 0 {
            let mut registered_root_chunks = match global_universe.registered_root_chunks.lock() {
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

        let parent_chunk = match global_universe.get_registered_chunk(&parent_chunk_id) {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((UnregisterChunkError::FailedToGetParentChunk, chunk_id));
            }
        };
        let parent_chunk = match parent_chunk {
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

        let parent_chunk_metadata = match Self::get_chunk_metadata_mut(&mut parent_chunk) {
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
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
        chunk_metadata: ChunkMetadata,
    ) -> Result<LoadChunkMetadataSuccess, (LoadChunkMetadataError, ChunkID, ChunkMetadata)> {
        let chunk = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((
                    LoadChunkMetadataError::FailedToGetChunk,
                    chunk_id,
                    chunk_metadata,
                ));
            }
        };
        let chunk = match chunk {
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
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
    ) -> Result<UnloadChunkMetadataSuccess, (UnloadChunkMetadataError, ChunkID)> {
        let chunk = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((UnloadChunkMetadataError::FailedToGetChunk, chunk_id));
            }
        };
        let chunk = match chunk {
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

        let chunk_metadata = match Self::get_chunk_metadata(&chunk) {
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
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
        chunk_data: ChunkData,
    ) -> Result<LoadChunkDataSuccess, (LoadChunkDataError, ChunkID, ChunkData)> {
        let chunk = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((LoadChunkDataError::FailedToGetChunk, chunk_id, chunk_data));
            }
        };
        let chunk = match chunk {
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

        let (stolen_id, stolen_metadata) = match &mut *chunk {
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

                (stolen_id, stolen_metadata)
            }
            Chunk::DataLoaded { .. } => {
                return Err((
                    LoadChunkDataError::ChunkDataAlreadyLoaded,
                    chunk_id,
                    chunk_data,
                ));
            }
        };

        let parent_chunk = stolen_metadata.parent_chunk.clone();

        if let Some(parent_chunk) = parent_chunk {
            let parent_chunk = match parent_chunk.lock() {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((
                        LoadChunkDataError::ParentChunkMutexPoisoned,
                        chunk_id,
                        chunk_data,
                    ));
                }
            };

            match *parent_chunk {
                Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                    return Err((
                        LoadChunkDataError::ParentChunkDataNotLoaded,
                        chunk_id,
                        chunk_data,
                    ));
                }
                Chunk::DataLoaded { .. } => {}
            }
        }

        *chunk = Chunk::DataLoaded {
            id: stolen_id,
            metadata: stolen_metadata,
            data: chunk_data,
        };
        return Ok(LoadChunkDataSuccess);
    }

    fn unload_chunk_data(
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
    ) -> Result<UnloadChunkDataSuccess, (UnloadChunkDataError, ChunkID)> {
        let chunk = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((UnloadChunkDataError::FailedToGetChunk, chunk_id));
            }
        };
        let chunk = match chunk {
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

        let chunk_data = match Self::get_chunk_data(&chunk) {
            Ok(chunk_data) => chunk_data,
            Err(_) => {
                return Err((UnloadChunkDataError::ChunkDataAlreadyUnloaded, chunk_id));
            }
        };
        if chunk_data.run_state != ChunkRunState::Despawned {
            return Err((UnloadChunkDataError::ChunkStillSpawned, chunk_id));
        }

        let (stolen_id, stolen_metadata) = match &mut *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err((UnloadChunkDataError::ChunkDataAlreadyUnloaded, chunk_id));
            }
            Chunk::DataLoaded { id, metadata, .. } => {
                let stolen_id = std::mem::take(id);
                let stolen_metadata = std::mem::take(metadata);

                (stolen_id, stolen_metadata)
            }
        };

        if let Some(ref chunk_child_chunks) = stolen_metadata.child_chunks {
            if !chunk_child_chunks.is_empty() {
                return Err((UnloadChunkDataError::ChildChunksStillRegistered, chunk_id));
            }
        }

        *chunk = Chunk::MetadataLoaded {
            id: stolen_id,
            metadata: stolen_metadata,
        };
        return Ok(UnloadChunkDataSuccess);
    }

    fn spawn_chunk(
        commands: &mut Commands,
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
    ) -> Result<SpawnChunkSuccess, (SpawnChunkError, ChunkID)> {
        let chunk_mutex = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk_mutex) => chunk_mutex,
            Err(_) => {
                return Err((SpawnChunkError::FailedToGetChunk, chunk_id));
            }
        };
        let chunk_mutex = match chunk_mutex {
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

        let (chunk_metadata, chunk_data) = match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err((SpawnChunkError::ChunkDataNotLoaded, chunk_id));
            }
            Chunk::DataLoaded {
                ref metadata,
                ref mut data,
                ..
            } => (metadata, data),
        };

        if let Some(ref parent_chunk) = chunk_metadata.parent_chunk {
            let parent_chunk = match parent_chunk.lock() {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((SpawnChunkError::ParentChunkMutexPoisoned, chunk_id));
                }
            };

            let parent_chunk_data = match Self::get_chunk_data(&parent_chunk) {
                Ok(parent_chunk_data) => parent_chunk_data,
                Err(_) => {
                    return Err((SpawnChunkError::ParentChunkNotSpawned, chunk_id));
                }
            };

            match parent_chunk_data.run_state {
                ChunkRunState::Despawned => {
                    return Err((SpawnChunkError::ParentChunkNotSpawned, chunk_id));
                }
                ChunkRunState::Spawned { .. } => {}
            }
        };

        match chunk_data.run_state {
            ChunkRunState::Despawned => {
                chunk_data.run_state = ChunkRunState::Spawned {
                    bevy_entity: commands
                        .spawn(ChunkBevyComponent {
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
        global_universe: &mut GlobalUniverse,
        chunk_id: ChunkID,
    ) -> Result<DespawnChunkSuccess, (DespawnChunkError, ChunkID)> {
        let chunk = match global_universe.get_registered_chunk(&chunk_id) {
            Ok(chunk) => chunk,
            Err(_) => {
                return Err((DespawnChunkError::FailedToGetChunk, chunk_id));
            }
        };
        let chunk = match chunk {
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

        let (chunk_metadata, chunk_data) = match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err((DespawnChunkError::ChunkAlreadyDespawned, chunk_id));
            }
            Chunk::DataLoaded {
                ref metadata,
                ref mut data,
                ..
            } => (metadata, data),
        };

        if let Some(ref chunk_child_chunks) = chunk_metadata.child_chunks {
            if !chunk_child_chunks.is_empty() {
                return Err((DespawnChunkError::ChildChunksStillSpawned, chunk_id));
            }
        };

        match chunk_data.run_state {
            ChunkRunState::Despawned => {
                return Err((DespawnChunkError::ChunkAlreadyDespawned, chunk_id));
            }
            ChunkRunState::Spawned {
                bevy_entity: ecs_entity,
            } => {
                commands.entity(ecs_entity).despawn();
                chunk_data.run_state = ChunkRunState::Despawned;
                return Ok(DespawnChunkSuccess);
            }
        }
    }

    fn register_entity(
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<RegisterEntitySuccess, (RegisterEntityError, EntityID)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((RegisterEntityError::FailedToGetParentChunk, entity_id));
                }
            };

        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((RegisterEntityError::ParentChunkNotRegistered, entity_id));
            }
        };

        let mut parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((RegisterEntityError::ParentChunkMutexPoisoned, entity_id));
            }
        };

        let parent_chunk_metadata = match Self::get_chunk_metadata_mut(&mut parent_chunk) {
            Ok(parent_chunk_metadata) => parent_chunk_metadata,
            Err(_) => {
                return Err((RegisterEntityError::ParentChunkMetadataNotLoaded, entity_id));
            }
        };

        let local_entity_id = entity_id.get_local_id();

        if parent_chunk_metadata
            .registered_entities
            .contains_key(local_entity_id)
        {
            return Err((RegisterEntityError::EntityAlreadyRegistered, entity_id));
        }

        let entity = Arc::new(Mutex::new(entity::Entity::new(entity_id.clone())));

        parent_chunk_metadata
            .registered_entities
            .insert(local_entity_id.clone(), entity);

        return Ok(RegisterEntitySuccess);
    }

    fn unregister_entity(
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<UnregisterEntitySuccess, (UnregisterEntityError, EntityID)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((UnregisterEntityError::FailedToGetParentChunk, entity_id));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((UnregisterEntityError::ParentChunkNotRegistered, entity_id));
            }
        };
        let mut parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((UnregisterEntityError::ParentChunkMutexPoisoned, entity_id));
            }
        };

        let parent_chunk_metadata = match Self::get_chunk_metadata_mut(&mut parent_chunk) {
            Ok(parent_chunk_metadata) => parent_chunk_metadata,
            Err(_) => {
                return Err((
                    UnregisterEntityError::ParentChunkMetadataNotLoaded,
                    entity_id,
                ));
            }
        };

        let local_entity_id = entity_id.get_local_id();

        match parent_chunk_metadata
            .registered_entities
            .remove(local_entity_id)
        {
            Some(_) => {
                return Ok(UnregisterEntitySuccess);
            }
            None => {
                return Err((UnregisterEntityError::EntityAlreadyUnregistered, entity_id));
            }
        };
    }

    fn load_entity_metadata(
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
        metadata: EntityMetadata,
    ) -> Result<LoadEntityMetadataSuccess, (LoadEntityMetadataError, EntityID, EntityMetadata)>
    {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((
                        LoadEntityMetadataError::FailedToGetParentChunk,
                        entity_id,
                        metadata,
                    ));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((
                    LoadEntityMetadataError::ParentChunkNotRegistered,
                    entity_id,
                    metadata,
                ));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((
                    LoadEntityMetadataError::ParentChunkMutexPoisoned,
                    entity_id,
                    metadata,
                ));
            }
        };

        let entity = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity) => entity,
            Err(_) => {
                return Err((
                    LoadEntityMetadataError::FailedToGetEntity,
                    entity_id,
                    metadata,
                ));
            }
        };
        let entity = match entity {
            Some(entity) => entity,
            None => {
                return Err((
                    LoadEntityMetadataError::EntityNotRegistered,
                    entity_id,
                    metadata,
                ));
            }
        };
        let mut entity = match entity.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((
                    LoadEntityMetadataError::EntityMutexPoisoned,
                    entity_id,
                    metadata,
                ));
            }
        };

        match &mut *entity {
            entity::Entity::Registered { .. } => {
                let mut stolen_entity = std::mem::take(&mut *entity);
                if let entity::Entity::Registered { id } = &mut stolen_entity {
                    let stolen_id = std::mem::take(&mut *id);
                    *entity = entity::Entity::MetadataLoaded {
                        id: stolen_id,
                        metadata,
                    };
                    return Ok(LoadEntityMetadataSuccess);
                } else {
                    return Err((
                        LoadEntityMetadataError::FatalUnexpectedError,
                        entity_id,
                        metadata,
                    ));
                }
            }
            entity::Entity::MetadataLoaded { .. } | entity::Entity::DataLoaded { .. } => {
                return Err((
                    LoadEntityMetadataError::EntityMetadataAlreadyLoaded,
                    entity_id,
                    metadata,
                ));
            }
        }
    }

    fn unload_entity_metadata(
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<UnloadEntityMetadataSuccess, (UnloadEntityMetadataError, EntityID)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((UnloadEntityMetadataError::FailedToGetParentChunk, entity_id));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((
                    UnloadEntityMetadataError::ParentChunkNotRegistered,
                    entity_id,
                ));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((
                    UnloadEntityMetadataError::ParentChunkMutexPoisoned,
                    entity_id,
                ));
            }
        };

        let entity = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity) => entity,
            Err(_) => {
                return Err((UnloadEntityMetadataError::FailedToGetEntity, entity_id));
            }
        };
        let entity = match entity {
            Some(entity) => entity,
            None => {
                return Err((UnloadEntityMetadataError::EntityNotRegistered, entity_id));
            }
        };
        let mut entity = match entity.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((UnloadEntityMetadataError::EntityMutexPoisoned, entity_id));
            }
        };

        match &mut *entity {
            entity::Entity::Registered { .. } => {
                return Err((
                    UnloadEntityMetadataError::EntityMetadataAlreadyUnloaded,
                    entity_id,
                ));
            }
            entity::Entity::MetadataLoaded { id, .. } => {
                let stolen_id = std::mem::take(id);
                *entity = entity::Entity::Registered { id: stolen_id };
                return Ok(UnloadEntityMetadataSuccess);
            }
            entity::Entity::DataLoaded { .. } => {
                return Err((UnloadEntityMetadataError::EntityDataStillLoaded, entity_id));
            }
        }
    }

    fn load_entity_data(
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
        data: EntityData,
    ) -> Result<LoadEntityDataSuccess, (LoadEntityDataError, EntityID, EntityData)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((LoadEntityDataError::FailedToGetParentChunk, entity_id, data));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((
                    LoadEntityDataError::ParentChunkNotRegistered,
                    entity_id,
                    data,
                ));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((
                    LoadEntityDataError::ParentChunkMutexPoisoned,
                    entity_id,
                    data,
                ));
            }
        };

        match Self::get_chunk_load_state(&parent_chunk) {
            ChunkLoadState::Registered | ChunkLoadState::MetadataLoaded => {
                return Err((
                    LoadEntityDataError::ParentChunkDataNotLoaded,
                    entity_id,
                    data,
                ));
            }
            ChunkLoadState::DataLoaded => {}
        }

        let entity = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity) => entity,
            Err(_) => {
                return Err((LoadEntityDataError::FailedToGetEntity, entity_id, data));
            }
        };
        let entity = match entity {
            Some(entity) => entity,
            None => {
                return Err((LoadEntityDataError::EntityNotRegistered, entity_id, data));
            }
        };
        let mut entity = match entity.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((LoadEntityDataError::EntityMutexPoisoned, entity_id, data));
            }
        };

        match &mut *entity {
            entity::Entity::Registered { .. } => {
                return Err((LoadEntityDataError::EntityMetadataNotLoaded, entity_id, data));
            },
            entity::Entity::MetadataLoaded { id, metadata, .. } => {
                let stolen_id = std::mem::take(id);
                let stolen_metadata = std::mem::take(metadata);

                *entity = entity::Entity::DataLoaded {
                    id: stolen_id,
                    metadata: stolen_metadata,
                    data,
                };
                return Ok(LoadEntityDataSuccess);
            }
            entity::Entity::DataLoaded { id, metadata, .. } => {
                return Err((LoadEntityDataError::EntityDataAlreadyLoaded, entity_id, data));
            }
        }
    }

    fn unload_entity_data(
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<UnloadEntityDataSuccess, (UnloadEntityDataError, EntityID)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((UnloadEntityDataError::FailedToGetParentChunk, entity_id));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((UnloadEntityDataError::ParentChunkNotRegistered, entity_id));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((UnloadEntityDataError::ParentChunkMutexPoisoned, entity_id));
            }
        };

        let entity = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity) => entity,
            Err(_) => {
                return Err((UnloadEntityDataError::FailedToGetEntity, entity_id));
            }
        };
        let entity = match entity {
            Some(entity) => entity,
            None => {
                return Err((UnloadEntityDataError::EntityNotRegistered, entity_id));
            }
        };
        let mut entity = match entity.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((UnloadEntityDataError::EntityMutexPoisoned, entity_id));
            }
        };

        match &mut *entity {
            entity::Entity::Registered { .. } | entity::Entity::MetadataLoaded { .. } => {
                return Err((UnloadEntityDataError::EntityDataAlreadyUnloaded, entity_id));
            }
            entity::Entity::DataLoaded { id, metadata, .. } => {
                let stolen_id = std::mem::take(id);
                let stolen_metadata = std::mem::take(metadata);

                *entity = entity::Entity::MetadataLoaded {
                    id: stolen_id,
                    metadata: stolen_metadata,
                };
                return Ok(UnloadEntityDataSuccess);
            }
        }
    }

    fn spawn_entity(
        commands: &mut Commands,
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<SpawnEntitySuccess, (SpawnEntityError, EntityID)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((SpawnEntityError::FailedToGetParentChunk, entity_id));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((SpawnEntityError::ParentChunkNotRegistered, entity_id));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((SpawnEntityError::ParentChunkMutexPoisoned, entity_id));
            }
        };

        let parent_chunk_data = match Self::get_chunk_data(&parent_chunk) {
            Ok(parent_chunk_data) => parent_chunk_data,
            Err(_) => {
                return Err((SpawnEntityError::ParentChunkDataNotLoaded, entity_id));
            }
        };

        match parent_chunk_data.run_state {
            ChunkRunState::Despawned => {
                return Err((SpawnEntityError::ParentChunkNotSpawned, entity_id));
            }
            ChunkRunState::Spawned { .. } => {}
        }

        let entity_mutex = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity_mutex) => entity_mutex,
            Err(_) => {
                return Err((SpawnEntityError::FailedToGetEntity, entity_id));
            }
        };
        let entity_mutex = match entity_mutex {
            Some(entity_mutex) => entity_mutex,
            None => {
                return Err((SpawnEntityError::EntityNotRegistered, entity_id));
            }
        };
        let mut entity = match entity_mutex.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((SpawnEntityError::EntityMutexPoisoned, entity_id));
            }
        };

        let entity_data = match Self::get_entity_data_mut(&mut entity) {
            Ok(entity_data) => entity_data,
            Err(_) => {
                return Err((SpawnEntityError::EntityDataNotLoaded, entity_id));
            }
        };

        match entity_data.run_state {
            EntityRunState::Despawned => {
                entity_data.run_state = EntityRunState::Spawned {
                    bevy_entity: commands
                        .spawn(EntityBevyComponent {
                            entity: entity_mutex.clone(),
                        })
                        .id(),
                };
                return Ok(SpawnEntitySuccess);
            }
            EntityRunState::Spawned { .. } => {
                return Err((SpawnEntityError::EntityAlreadySpawned, entity_id));
            }
        }
    }

    fn despawn_entity(
        commands: &mut Commands,
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<DespawnEntitySuccess, (DespawnEntityError, EntityID)> {
        let parent_chunk =
            match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    return Err((DespawnEntityError::FailedToGetParentChunk, entity_id));
                }
            };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((DespawnEntityError::ParentChunkNotRegistered, entity_id));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((DespawnEntityError::ParentChunkMutexPoisoned, entity_id));
            }
        };

        let entity = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity) => entity,
            Err(_) => {
                return Err((DespawnEntityError::FailedToGetEntity, entity_id));
            }
        };
        let entity = match entity {
            Some(entity) => entity,
            None => {
                return Err((DespawnEntityError::EntityNotRegistered, entity_id));
            }
        };
        let mut entity = match entity.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((DespawnEntityError::EntityMutexPoisoned, entity_id));
            }
        };

        let entity_data = match Self::get_entity_data_mut(&mut entity) {
            Ok(entity_data) => entity_data,
            Err(_) => {
                return Err((DespawnEntityError::EntityDataNotLoaded, entity_id));
            }
        };

        match entity_data.run_state {
            EntityRunState::Despawned => {
                return Err((DespawnEntityError::EntityAlreadyDespawned, entity_id));
            }
            EntityRunState::Spawned {
                bevy_entity: ecs_entity,
            } => {
                commands.entity(ecs_entity).despawn();
                entity_data.run_state = EntityRunState::Despawned;
                return Ok(DespawnEntitySuccess);
            }
        }
    }

    fn command_entity(
        command: Box<dyn FnOnce(&mut EntityCommands) + Send>,
        commands: &mut Commands,
        global_universe: &mut GlobalUniverse,
        entity_id: EntityID,
    ) -> Result<CommandEntitySuccess, (CommandEntityError, EntityID)> {
        let parent_chunk = match global_universe.get_registered_chunk(&entity_id.get_parent_chunk_id()) {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((CommandEntityError::FailedToGetParentChunk, entity_id));
            }
        };
        let parent_chunk = match parent_chunk {
            Some(parent_chunk) => parent_chunk,
            None => {
                return Err((CommandEntityError::ParentChunkNotRegistered, entity_id));
            }
        };
        let parent_chunk = match parent_chunk.lock() {
            Ok(parent_chunk) => parent_chunk,
            Err(_) => {
                return Err((CommandEntityError::ParentChunkMutexPoisoned, entity_id));
            }
        };

        let entity = match Self::get_registered_entity(&parent_chunk, &entity_id) {
            Ok(entity) => entity,
            Err(_) => {
                return Err((CommandEntityError::FailedToGetEntity, entity_id));
            }
        };
        let entity = match entity {
            Some(entity) => entity,
            None => {
                return Err((CommandEntityError::EntityNotRegistered, entity_id));
            }
        };
        let mut entity = match entity.lock() {
            Ok(entity) => entity,
            Err(_) => {
                return Err((CommandEntityError::EntityMutexPoisoned, entity_id));
            }
        };

        let entity_data = match Self::get_entity_data_mut(&mut entity) {
            Ok(entity_data) => entity_data,
            Err(_) => {
                return Err((CommandEntityError::EntityDataNotLoaded, entity_id));
            }
        };

        let entity_bevy_entity = match entity_data.run_state {
            EntityRunState::Despawned => {
                return Err((CommandEntityError::EntityNotSpawned, entity_id));
            }
            EntityRunState::Spawned { bevy_entity } => bevy_entity
        };

        command(&mut commands.entity(entity_bevy_entity));

        return Ok(CommandEntitySuccess);
    }
}

// Module Functions
