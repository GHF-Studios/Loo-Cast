// Modules
pub mod id;

// Local imports

// Internal imports
use crate::engine::kernel::game::SimulationState;
use crate::engine::kernel::universe::chunk::data::*;
use crate::engine::kernel::universe::chunk::id::*;
use crate::engine::kernel::universe::chunk::metadata::*;
use crate::engine::kernel::universe::chunk::pos::*;
use crate::engine::kernel::universe::chunk::*;
use crate::engine::kernel::universe::entity::data::*;
use crate::engine::kernel::universe::entity::id::*;
use crate::engine::kernel::universe::entity::metadata::*;
use crate::engine::kernel::universe::entity::*;
use crate::engine::kernel::universe::*;
use crate::engine::kernel::AppState;

// External imports
use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums
pub enum OperationRequest {
    Chunk(ChunkOperationRequest),
    Entity(EntityOperationRequest),
}

// Structs
pub struct GlobalUniversePlugin;

pub struct GlobalUniverse {
    pub(in crate::engine::kernel::universe) registered_root_chunks:
        HashMap<LocalChunkID, Arc<Mutex<Chunk>>>,
    pub(in crate::engine::kernel::universe) operation_requests: Arc<Mutex<Vec<OperationRequest>>>,
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
        let (parent_chunk_id, parent_chunk_data) = match parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(
                    "Generating a local entity id failed: Parent chunk data is not loaded."
                        .to_string(),
                );
            }
            Chunk::DataLoaded { id, data, .. } => {
                (id.clone(), data)
            }
        };

        let local_entity_id = if !parent_chunk_data.recycled_local_entity_ids.is_empty() {
            parent_chunk_data.recycled_local_entity_ids.pop().unwrap()
        } else {
            let local_entity_id = parent_chunk_data.current_local_entity_id;

            parent_chunk_data.current_local_entity_id += 1;
            
            local_entity_id
        };

        let local_entity_id = match LocalEntityID::new(local_entity_id) {
            Ok(local_entity_id) => local_entity_id,
            Err(error) => {
                return Err(format!(
                    "Generating a local entity id failed: {}",
                    error
                ));
            }
        };
            
        let entity_id = EntityID::new(parent_chunk_id, local_entity_id);

        Ok(entity_id)
    }

    pub fn recycle_entity_id(parent_chunk: &mut Chunk, entity_id: EntityID) -> Result<(), String> {
        let chunk_data = match parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(
                    "Recycling a local entity id failed: Parent chunk data is not loaded."
                        .to_string(),
                );
            }
            Chunk::DataLoaded { data, .. } => data,
        };

        if chunk_data
            .recycled_local_entity_ids
            .contains(&entity_id.get_local_entity_id().get_id())
        {
            return Err("Entity id already recycled.".to_string());
        }

        chunk_data
            .recycled_local_entity_ids
            .push(entity_id.get_local_entity_id().get_id());

        Ok(())
    }

    pub fn get_registered_chunk(&self,chunk_id: &ChunkID,) -> Result<Option<Arc<Mutex<Chunk>>>, String> {
        let mut path = chunk_id.get_global_id_base10x10().clone();

        if path.is_empty() {
            return Err("Failed to get registered chunk: Invalid chunk id.".to_string());
        }

        let root_local_chunk_id: LocalChunkID = match path.remove(0).try_into() {
            Ok(root_local_chunk_id) => root_local_chunk_id,
            Err(_) => {
                return Err(
                    "Failed to check if chunk is registered: Invalid root local chunk id."
                        .to_string(),
                )
            }
        };

        let mut registered_chunk = match self.registered_root_chunks
            .get(&root_local_chunk_id)
        {
            Some(registered_chunk) => registered_chunk.clone(),
            None => {
                return Err("Failed to get registered chunk: Root chunk not registered.".to_string())
            }
        };

        for &local_chunk_id in &path {
            let local_chunk_id: LocalChunkID = match local_chunk_id.try_into() {
                Ok(local_chunk_id) => local_chunk_id,
                Err(_) => {
                    return Err(
                        "Failed to get registered chunk: Invalid local chunk id."
                            .to_string(),
                    )
                }
            };

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

                let current_chunk_data =
                    match *current_chunk {
                        Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => return Err(
                            "Failed to get registered chunk: Current chunk data not loaded."
                                .to_string(),
                        ),
                        Chunk::DataLoaded { ref data, .. } => data,
                    };

                let current_chunk_child_chunks = match current_chunk_data.child_chunks {
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

        let root_local_chunk_id: LocalChunkID = match path.remove(0).try_into() {
            Ok(root_local_chunk_id) => root_local_chunk_id,
            Err(_) => {
                return Err(
                    "Failed to check if chunk is registered: Invalid root local chunk id."
                        .to_string(),
                )
            }
        };

        let mut registered_chunk = match self.registered_root_chunks.get(&root_local_chunk_id) {
                Some(registered_chunk) => registered_chunk.clone(),
                None => {
                    return Err(
                        "Failed to check if chunk is registered: Root chunk not registered."
                            .to_string(),
                    )
                }
            };

        for local_chunk_id in path {
            let local_chunk_id: LocalChunkID = match local_chunk_id.try_into() {
                Ok(local_chunk_id) => local_chunk_id,
                Err(_) => {
                    return Err(
                        "Failed to check if chunk is registered: Invalid local chunk id."
                            .to_string(),
                    )
                }
            };

            let next_chunk = {
                let current_chunk =
                    match registered_chunk.lock() {
                        Ok(current_chunk) => current_chunk,
                        Err(_) => return Err(
                            "Failed to check if chunk is registered: Current chunk mutex poisoned."
                                .to_string(),
                        ),
                    };

                let current_chunk_data = match *current_chunk {
                    Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => return Err(
                        "Failed to check if chunk is registered: Current chunk data not loaded."
                        .to_string()
                    ),
                    Chunk::DataLoaded { ref data, .. } => data,
                };

                let current_chunk_child_chunks = match current_chunk_data.child_chunks {
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
        let parent_chunk_data = match parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(
                    "Failed to get registered entity: Parent chunk data is not loaded."
                        .to_string(),
                );
            }
            Chunk::DataLoaded { data, .. } => data,
        };

        match parent_chunk_data
            .registered_entities
            .get(&entity_id.get_local_entity_id())
        {
            Some(registered_entity) => Ok(Some(registered_entity.clone())),
            None => Ok(None),
        }
    }

    pub fn is_entity_registered(
        parent_chunk: &Chunk,
        entity_id: &EntityID,
    ) -> Result<bool, String> {
        let parent_chunk_data = match parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(
                    "Failed to check if entity is registered: Parent chunk data is not loaded."
                        .to_string(),
                );
            }
            Chunk::DataLoaded { data, .. } => data,
        };

        Ok(parent_chunk_data.registered_entities.contains_key(&entity_id.get_local_entity_id()))
    }

    pub fn send_chunk_operation_request(
        &mut self,
        request: ChunkOperationRequest,
    ) -> Result<(), String> {
        let mut operation_requests =
            match self.operation_requests.lock() {
                Ok(operation_requests) => operation_requests,
                Err(_) => return Err(
                    "Failed to request chunk operation: Operation requests mutex poisoned."
                        .to_string(),
                ),
            };
        operation_requests.push(OperationRequest::Chunk(request));
        Ok(())
    }

    pub fn send_entity_operation_request(
        &mut self,
        request: EntityOperationRequest,
    ) -> Result<(), String> {
        let mut operation_reuests =
            match self.operation_requests.lock() {
                Ok(operation_reuests) => operation_reuests,
                Err(_) => return Err(
                    "Failed to request entity operation: Operation requests mutex poisoned."
                        .to_string(),
                ),
            };
            operation_reuests.push(OperationRequest::Entity(request));
        Ok(())
    }

    pub fn get_chunk_load_state(chunk: &Chunk) -> ChunkLoadState {
        match *chunk {
            Chunk::Registered { .. } => ChunkLoadState::Registered,
            Chunk::MetadataLoaded { .. } => ChunkLoadState::MetadataLoaded,
            Chunk::DataLoaded { .. } => ChunkLoadState::DataLoaded,
        }
    }

    pub fn get_entity_load_state(entity: &entity::Entity) -> EntityLoadState {
        match *entity {
            entity::Entity::Registered { .. } => EntityLoadState::Registered,
            entity::Entity::MetadataLoaded { .. } => EntityLoadState::MetadataLoaded,
            entity::Entity::DataLoaded { .. } => EntityLoadState::DataLoaded,
        }
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
    
        let mut global_universe_operation_requests =
            global_universe.operation_requests.lock().unwrap_or_else(|_| {
                panic!(
                    "Failed to handle operation requests: Operation requests mutex poisoned."
                )
            });

        let mut operation_requests = Vec::new();
        operation_requests.append(&mut *global_universe_operation_requests);

        drop(global_universe_operation_requests);

        global_universe.process_operation_requests(operation_requests, &mut commands);
    }

    fn process_operation_requests(
        &mut self,
        operation_requests: Vec<OperationRequest>,
        commands: &mut Commands,
    ) {
        for operation_request in operation_requests {
            match operation_request {
                OperationRequest::Chunk(chunk_operation_request) => {
                    self.process_chunk_operations(chunk_operation_request, commands);
                },
                OperationRequest::Entity(entity_operation_request) => {
                    Self::process_entity_operations(entity_operation_request, commands);
                },
            }
        }
    }

    fn process_chunk_operations(
        &mut self,
        chunk_operation_request: ChunkOperationRequest,
        commands: &mut Commands,
    ) {
        for chunk_operation in chunk_operation_request.operations {
            match chunk_operation {
                ChunkOperation::Register {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::register_chunk(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::Unregister {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::unregister_chunk(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::LoadMetadata {
                    id,
                    metadata,
                    success_callback,
                    failure_callback,
                } => match Self::load_chunk_metadata(id, metadata) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::UnloadMetadata {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::unload_chunk_metadata(id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::LoadData {
                    id,
                    data,
                    success_callback,
                    failure_callback,
                } => match Self::load_chunk_data(id, data) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::UnloadData {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::unload_chunk_data(id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::Spawn {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::spawn_chunk(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                ChunkOperation::Despawn {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::despawn_chunk(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
            }
        }
    }

    fn process_entity_operations(
        entity_operation_request: EntityOperationRequest,
        commands: &mut Commands,
    ) {
        for entity_operation in entity_operation_request.operations {
            match entity_operation {
                EntityOperation::Register {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::register_entity(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::Unregister {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::unregister_entity(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::LoadMetadata {
                    id,
                    metadata,
                    success_callback,
                    failure_callback,
                } => match Self::load_entity_metadata(id, metadata) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::UnloadMetadata {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::unload_entity_metadata(id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::LoadData {
                    id,
                    data,
                    success_callback,
                    failure_callback,
                } => match Self::load_entity_data(id, data) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::UnloadData {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::unload_entity_data(id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::Spawn {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::spawn_entity(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::Despawn {
                    id,
                    success_callback,
                    failure_callback,
                } => match Self::despawn_entity(commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }
                },
                EntityOperation::Command { 
                    id, 
                    command, 
                    success_callback, 
                    failure_callback 
                } => match Self::command_entity(command, commands, id) {
                    Ok(success) => {
                        success_callback(success);
                    }
                    Err(error) => {
                        failure_callback(error);
                    }

                }
            }
        }
    }

    fn register_root_chunk(
        &mut self,
        commands: &mut Commands,
        local_chunk_id: LocalChunkID,
    ) -> Result<RegisterRootChunkSuccess, RegisterRootChunkError> {
        if self.registered_root_chunks.contains_key(&local_chunk_id) {
            return Err(RegisterRootChunkError::ChunkAlreadyRegistered);
        }

        let chunk_id = match ChunkID::new_root_id(local_chunk_id) {
            Ok(chunk_id) => chunk_id,
            Err(_) => {
                return Err(RegisterRootChunkError::FailedToCreateChunkID);
            }
        };

        let chunk_bevy_entity = commands.spawn(()).id();

        let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id.clone(), chunk_bevy_entity)));

        commands.entity(chunk_bevy_entity).insert(ChunkBevyComponent {
            chunk: chunk.clone(),
        });

        self.registered_root_chunks.insert(local_chunk_id, chunk);

        return Ok(RegisterRootChunkSuccess);
    }

    fn register_chunk(
        commands: &mut Commands,
        parent_chunk: &mut Chunk,
        local_chunk_id: LocalChunkID,
    ) -> Result<RegisterChunkSuccess, RegisterChunkError> {
        let (parent_chunk_id, parent_chunk_data) = match *parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(RegisterChunkError::ParentChunkDataNotLoaded);
            },
            Chunk::DataLoaded { ref id, ref mut data, .. } => (id, data),
        };

        let parent_chunk_child_chunks = match parent_chunk_data.child_chunks {
            Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err(RegisterChunkError::ParentChunkNotAllowedToHaveChildChunks);
            }
        };

        if parent_chunk_child_chunks.contains_key(&local_chunk_id) {
            return Err(RegisterChunkError::ChunkAlreadyRegistered);
        }

        let chunk_id = match ChunkID::new_id(&parent_chunk_id.clone(), local_chunk_id) {
            Ok(chunk_id) => chunk_id,
            Err(_) => {
                return Err(RegisterChunkError::FailedToCreateChunkID);
            }
        };

        let chunk_bevy_entity = commands.spawn(()).id();

        let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id.clone(), chunk_bevy_entity)));

        commands.entity(chunk_bevy_entity).insert(ChunkBevyComponent {
            chunk: chunk.clone(),
        });

        parent_chunk_child_chunks.insert(local_chunk_id, chunk);

        return Ok(RegisterChunkSuccess);
    }

    fn unregister_root_chunk(
        &mut self,
        commands: &mut Commands,
        chunk: &Chunk
    ) -> Result<UnregisterRootChunkSuccess, UnregisterRootChunkError> {
        let (chunk_id, chunk_bevy_entity) = match *chunk {
            Chunk::Registered { ref id, ref bevy_entity } => (id.clone(), bevy_entity.clone()),
            Chunk::MetadataLoaded { .. } => {
                return Err(UnregisterRootChunkError::ChunkMetadataStillLoaded);
            },
            Chunk::DataLoaded { .. } => {
                return Err(UnregisterRootChunkError::ChunkDataStillLoaded);
            }
        };

        let local_chunk_id = match chunk_id.compute_local_chunk_id() {
            Ok(local_chunk_id) => local_chunk_id,
            Err(_) => {
                return Err(UnregisterRootChunkError::FailedToComputeLocalChunkID);
            }
        };

        if !self.registered_root_chunks.contains_key(&local_chunk_id) {
            return Err(UnregisterRootChunkError::ChunkAlreadyUnregistered);
        }

        self.registered_root_chunks.remove(&local_chunk_id);

        commands.entity(chunk_bevy_entity).despawn();

        return Ok(UnregisterRootChunkSuccess);
    }

    fn unregister_chunk(
        commands: &mut Commands,
        parent_chunk: &mut Chunk,
        chunk: &Chunk
    ) -> Result<UnregisterChunkSuccess, UnregisterChunkError> {
        let (chunk_id, chunk_bevy_entity) = match *chunk {
            Chunk::Registered { ref id, ref bevy_entity } => (id.clone(), bevy_entity.clone()),
            Chunk::MetadataLoaded { .. } => {
                return Err(UnregisterChunkError::ChunkMetadataStillLoaded);
            },
            Chunk::DataLoaded { .. } => {
                return Err(UnregisterChunkError::ChunkDataStillLoaded);
            }
        };

        let local_chunk_id = match chunk_id.compute_local_chunk_id() {
            Ok(local_chunk_id) => local_chunk_id,
            Err(_) => {
                return Err(UnregisterChunkError::FailedToComputeLocalChunkID);
            }
        };

        let parent_chunk_data = match *parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(UnregisterChunkError::ParentChunkDataNotLoaded);
            },
            Chunk::DataLoaded { ref mut data, .. } => data,
        };

        let parent_chunk_child_chunks = match parent_chunk_data.child_chunks {
            Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err(UnregisterChunkError::ParentChunkNotAllowedToHaveChildChunks);
            }
        };

        if !parent_chunk_child_chunks.contains_key(&local_chunk_id) {
            return Err(UnregisterChunkError::ChunkAlreadyUnregistered);
        }

        parent_chunk_child_chunks.remove(&local_chunk_id);

        commands.entity(chunk_bevy_entity).despawn();

        return Ok(UnregisterChunkSuccess);
    }

    fn load_chunk_metadata(
        chunk: &mut Chunk,
        chunk_metadata: ChunkMetadata,
    ) -> Result<LoadChunkMetadataSuccess, LoadChunkMetadataError> {
        match *chunk {
            Chunk::Registered { ref mut id, ref mut bevy_entity } => {
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                
                *chunk = Chunk::MetadataLoaded {
                    id: stolen_id,
                    bevy_entity: stolen_bevy_entity,
                    metadata: chunk_metadata,
                };
                
                Ok(LoadChunkMetadataSuccess)
            }
            Chunk::MetadataLoaded { .. } | Chunk::DataLoaded { .. } => {
                Err(LoadChunkMetadataError::ChunkMetadataAlreadyLoaded)
            }
        }
    }

    fn unload_chunk_metadata(
        chunk: &mut Chunk,
    ) -> Result<UnloadChunkMetadataSuccess, UnloadChunkMetadataError> {
        match *chunk {
            Chunk::Registered { .. } => {
                Err(UnloadChunkMetadataError::ChunkMetadataAlreadyUnloaded)
            }
            Chunk::MetadataLoaded { ref mut id, ref mut bevy_entity, .. } => {
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                
                *chunk = Chunk::Registered { id: stolen_id, bevy_entity: stolen_bevy_entity };
                
                Ok(UnloadChunkMetadataSuccess)
            }
            Chunk::DataLoaded { .. } => {
                Err(UnloadChunkMetadataError::ChunkDataStillLoaded)
            }
        }
    }

    fn load_chunk_data(
        chunk: &mut Chunk,
        chunk_data: ChunkData,
    ) -> Result<LoadChunkDataSuccess, LoadChunkDataError> {
        match *chunk {
            Chunk::Registered { .. } => {
                return Err(LoadChunkDataError::ChunkMetadataNotLoaded);
            }
            Chunk::MetadataLoaded { ref mut id, ref mut bevy_entity, ref mut metadata } => {
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                let stolen_metadata = std::mem::take(metadata);

                *chunk = Chunk::DataLoaded {
                    id: stolen_id,
                    bevy_entity: stolen_bevy_entity,
                    metadata: stolen_metadata,
                    data: chunk_data,
                };

                Ok(LoadChunkDataSuccess)
            }
            Chunk::DataLoaded { .. } => {
                return Err(LoadChunkDataError::ChunkDataAlreadyLoaded);
            }
        }
    }

    fn unload_chunk_data(
        chunk: &mut Chunk,
    ) -> Result<UnloadChunkDataSuccess, UnloadChunkDataError> {
        match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(UnloadChunkDataError::ChunkDataAlreadyUnloaded);
            }
            Chunk::DataLoaded { ref mut id, ref mut bevy_entity, ref mut metadata, ref mut data } => {
                if data.run_state == ChunkRunState::Spawned {
                    return Err(UnloadChunkDataError::ChunkStillSpawned);
                }

                if let Some(ref chunk_child_chunks) = data.child_chunks {
                    if !chunk_child_chunks.is_empty() {
                        return Err(UnloadChunkDataError::ChildChunksStillRegistered);
                    }
                }
        
                if !data.registered_entities.is_empty() {
                    return Err(UnloadChunkDataError::EntitiesStillRegistered);
                }

                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                let stolen_metadata = std::mem::take(metadata);
        
                *chunk = Chunk::MetadataLoaded {
                    id: stolen_id,
                    bevy_entity: stolen_bevy_entity,
                    metadata: stolen_metadata,
                };

                Ok(UnloadChunkDataSuccess)
            },
        }
    }

    fn spawn_chunk(
        _commands: &mut Commands,
        parent_chunk: &mut Chunk,
        chunk: &mut Chunk,
    ) -> Result<SpawnChunkSuccess, SpawnChunkError> {
        let parent_chunk_data = match *parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(SpawnChunkError::ParentChunkDataNotLoaded);
            }
            Chunk::DataLoaded { ref mut data, .. } => data,
        };

        match parent_chunk_data.run_state {
            ChunkRunState::Despawned => {
                return Err(SpawnChunkError::ParentChunkNotSpawned);
            }
            ChunkRunState::Spawned => {}
        }

        let parent_chunk_child_chunks = match parent_chunk_data.child_chunks {
            Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
            None => {
                return Err(SpawnChunkError::ParentChunkNotAllowedToHaveChildChunks);
            }
        };

        let (chunk_id, chunk_data) = match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(SpawnChunkError::ChunkDataNotLoaded);
            }
            Chunk::DataLoaded { ref id, ref mut data, .. } => (id, data),
        };

        match chunk_data.run_state {
            ChunkRunState::Despawned => {}
            ChunkRunState::Spawned => {
                return Err(SpawnChunkError::ChunkAlreadySpawned);
            }
        };

        let local_chunk_id = match chunk_id.compute_local_chunk_id() {
            Ok(local_chunk_id) => local_chunk_id,
            Err(_) => {
                return Err(SpawnChunkError::FailedToComputeLocalChunkID);
            }
        };

        if !parent_chunk_child_chunks.contains_key(&local_chunk_id) {
            return Err(SpawnChunkError::WrongParentChunk);
        }

        chunk_data.run_state = ChunkRunState::Spawned;

        Ok(SpawnChunkSuccess)
    }

    fn despawn_chunk(
        _commands: &mut Commands,
        chunk: &mut Chunk,
    ) -> Result<DespawnChunkSuccess, DespawnChunkError> {
        let chunk_data = match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(DespawnChunkError::ChunkDataNotLoaded);
            }
            Chunk::DataLoaded { ref mut data, .. } => data,
        };

        if chunk_data.run_state == ChunkRunState::Despawned {
            return Err(DespawnChunkError::ChunkAlreadyDespawned);
        }

        if let Some(child_chunks) = chunk_data.child_chunks {
            for child_chunk in child_chunks.values() {
                let child_chunk = match child_chunk.lock() {
                    Ok(child_chunk) => child_chunk,
                    Err(_) => {
                        return Err(DespawnChunkError::ChildChunkMutexPoisoned);
                    }
                };

                let child_chunk_data = match *child_chunk {
                    Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                        continue;
                    }
                    Chunk::DataLoaded { ref data, .. } => data,
                };

                match child_chunk_data.run_state {
                    ChunkRunState::Despawned => {}
                    ChunkRunState::Spawned => {
                        return Err(DespawnChunkError::ChildChunkStillSpawned);
                    }
                }
            }
        }

        for registered_entity in chunk_data.registered_entities.values() {
            let registered_entity = match registered_entity.lock() {
                Ok(registered_entity) => registered_entity,
                Err(_) => {
                    return Err(DespawnChunkError::RegisteredEntityMutexPoisoned);
                }
            };

            let registered_entity_data = match *registered_entity {
                entity::Entity::Registered { .. } | entity::Entity::MetadataLoaded { .. } => {
                    continue;
                }
                entity::Entity::DataLoaded { ref data, .. } => data,
            };

            match registered_entity_data.run_state {
                EntityRunState::Despawned => {}
                EntityRunState::Spawned => {
                    return Err(DespawnChunkError::RegisteredEntityStillSpawned);
                }
            }
        }

        chunk_data.run_state = ChunkRunState::Despawned;
        Ok(DespawnChunkSuccess)
    }

    fn register_entity(
        commands: &mut Commands,
        parent_chunk: &mut Chunk,
        local_entity_id: LocalEntityID,
    ) -> Result<RegisterEntitySuccess, RegisterEntityError> {
        let (parent_chunk_id, parent_chunk_data) = match *parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(RegisterEntityError::ParentChunkDataNotLoaded);
            }
            Chunk::DataLoaded { ref id, ref mut data, .. } => (id, data),
        };

        if parent_chunk_data
            .registered_entities
            .contains_key(&local_entity_id)
        {
            return Err(RegisterEntityError::EntityAlreadyRegistered);
        }

        let entity_id = EntityID::new(parent_chunk_id.clone(), local_entity_id);

        let entity_bevy_entity = commands.spawn(()).id();

        let entity = Arc::new(Mutex::new(entity::Entity::new(entity_id, entity_bevy_entity)));

        commands.entity(entity_bevy_entity).insert(EntityBevyComponent {
            entity: entity.clone(),
        });

        parent_chunk_data
            .registered_entities
            .insert(local_entity_id, entity);

        Ok(RegisterEntitySuccess)
    }

    fn unregister_entity(
        commands: &mut Commands,
        parent_chunk: &mut Chunk,
        entity: &mut crate::engine::kernel::universe::entity::Entity,
    ) -> Result<UnregisterEntitySuccess, UnregisterEntityError> {
        let parent_chunk_data = match *parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(UnregisterEntityError::ParentChunkDataNotLoaded);
            }
            Chunk::DataLoaded { ref mut data, .. } => data,
        };

        let (entity_id, entity_bevy_entity) = match *entity {
            entity::Entity::Registered { 
                ref id, 
                ref bevy_entity 
            } => (id.clone(), bevy_entity.clone()),
            entity::Entity::MetadataLoaded { .. } => {
                return Err(UnregisterEntityError::EntityMetadataStillLoaded);
            }
            entity::Entity::DataLoaded { .. } => {
                return Err(UnregisterEntityError::EntityDataStillLoaded);
            }
        };

        let local_entity_id = entity_id.get_local_entity_id();

        match parent_chunk_data.registered_entities.remove(&local_entity_id)
        {
            Some(_) => {
                commands.entity(entity_bevy_entity).despawn();

                Ok(UnregisterEntitySuccess)
            }
            None => {
                Err(UnregisterEntityError::EntityAlreadyUnregistered)
            }
        }
    }

    fn load_entity_metadata(
        entity: &mut crate::engine::kernel::universe::entity::Entity,
        entity_metadata: EntityMetadata,
    ) -> Result<LoadEntityMetadataSuccess, LoadEntityMetadataError> {
        match *entity {
            entity::Entity::Registered { ref mut id, ref mut bevy_entity } => {
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                
                *entity = entity::Entity::MetadataLoaded {
                    id: stolen_id,
                    bevy_entity: stolen_bevy_entity,
                    metadata: entity_metadata,
                };

                Ok(LoadEntityMetadataSuccess)
            }
            entity::Entity::MetadataLoaded { .. } | entity::Entity::DataLoaded { .. } => {
                Err(LoadEntityMetadataError::EntityMetadataAlreadyLoaded)
            }
        }
    }

    fn unload_entity_metadata(
        entity: &mut crate::engine::kernel::universe::entity::Entity,
    ) -> Result<UnloadEntityMetadataSuccess, UnloadEntityMetadataError> {
        match *entity {
            entity::Entity::Registered { .. } => {
                Err(UnloadEntityMetadataError::EntityMetadataAlreadyUnloaded)
            }
            entity::Entity::MetadataLoaded { ref mut id, ref mut bevy_entity, .. } => {
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                
                *entity = entity::Entity::Registered { id: stolen_id, bevy_entity: stolen_bevy_entity };
                
                Ok(UnloadEntityMetadataSuccess)
            }
            entity::Entity::DataLoaded { .. } => {
                Err(UnloadEntityMetadataError::EntityDataStillLoaded)
            }
        }
    }

    fn load_entity_data(
        entity: &mut crate::engine::kernel::universe::entity::Entity,
        entity_data: EntityData,
    ) -> Result<LoadEntityDataSuccess, LoadEntityDataError> {
        match *entity {
            entity::Entity::Registered { .. } => {
                Err(LoadEntityDataError::EntityMetadataNotLoaded)
            },
            entity::Entity::MetadataLoaded { ref mut id, ref mut bevy_entity, ref mut metadata } => {
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                let stolen_metadata = std::mem::take(metadata);

                *entity = entity::Entity::DataLoaded {
                    id: stolen_id,
                    bevy_entity: stolen_bevy_entity,
                    metadata: stolen_metadata,
                    data: entity_data,
                };
                
                Ok(LoadEntityDataSuccess)
            }
            entity::Entity::DataLoaded { .. } => {
                Err(LoadEntityDataError::EntityDataAlreadyLoaded)
            }
        }
    }

    fn unload_entity_data(
        entity: &mut crate::engine::kernel::universe::entity::Entity,
    ) -> Result<UnloadEntityDataSuccess, UnloadEntityDataError> {
        match *entity {
            entity::Entity::Registered { .. } | entity::Entity::MetadataLoaded { .. } => {
                Err(UnloadEntityDataError::EntityDataAlreadyUnloaded)
            }
            entity::Entity::DataLoaded { ref mut id, ref mut bevy_entity, ref mut metadata, ref data } => {
                if data.run_state == EntityRunState::Spawned {
                    return Err(UnloadEntityDataError::EntityStillSpawned);
                }
                
                let stolen_id = std::mem::take(id);
                let stolen_bevy_entity = std::mem::replace(bevy_entity, bevy::ecs::entity::Entity::PLACEHOLDER);
                let stolen_metadata = std::mem::take(metadata);

                *entity = entity::Entity::MetadataLoaded {
                    id: stolen_id,
                    bevy_entity: stolen_bevy_entity,
                    metadata: stolen_metadata,
                };

                Ok(UnloadEntityDataSuccess)
            }
        }
    }

    fn spawn_entity(
        _commands: &mut Commands,
        parent_chunk: &mut Chunk,
        entity: &mut crate::engine::kernel::universe::entity::Entity,
    ) -> Result<SpawnEntitySuccess, SpawnEntityError> {
        let parent_chunk_data = match *parent_chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => {
                return Err(SpawnEntityError::ParentChunkDataNotLoaded);
            }
            Chunk::DataLoaded { ref data, .. } => data,
        };

        match parent_chunk_data.run_state {
            ChunkRunState::Despawned => {
                return Err(SpawnEntityError::ParentChunkNotSpawned);
            }
            ChunkRunState::Spawned { .. } => {}
        }

        let (entity_id, entity_data) = match *entity {
            entity::Entity::Registered { .. } | entity::Entity::MetadataLoaded { .. } => {
                return Err(SpawnEntityError::EntityDataNotLoaded);
            }
            entity::Entity::DataLoaded { ref id, ref mut data, .. } => (id, data),
        };

        if entity_data.run_state == EntityRunState::Spawned {
            return Err(SpawnEntityError::EntityAlreadySpawned);
        }

        if !parent_chunk_data.registered_entities.contains_key(&entity_id.get_local_entity_id()) {
            return Err(SpawnEntityError::WrongParentChunk);
        }

        entity_data.run_state = EntityRunState::Spawned;

        Ok(SpawnEntitySuccess)
    }

    fn despawn_entity(
        _commands: &mut Commands,
        entity: &mut crate::engine::kernel::universe::entity::Entity,
    ) -> Result<DespawnEntitySuccess, DespawnEntityError> {
        let entity_data = match *entity {
            entity::Entity::Registered { .. } | entity::Entity::MetadataLoaded { .. } => {
                return Err(DespawnEntityError::EntityDataNotLoaded);
            }
            entity::Entity::DataLoaded { ref mut data, .. } => data,
        };

        match entity_data.run_state {
            EntityRunState::Despawned => {
                Err(DespawnEntityError::EntityAlreadyDespawned)
            }
            EntityRunState::Spawned => {
                entity_data.run_state = EntityRunState::Despawned;
                Ok(DespawnEntitySuccess)
            }
        }
    }

    fn command_entity(
        commands: &mut Commands,
        entity_commands: Box<dyn FnOnce(EntityCommands) + Send>,
        entity: &mut crate::engine::kernel::universe::entity::Entity,
    ) -> Result<CommandEntitySuccess, CommandEntityError> {
        match *entity {
            entity::Entity::Registered { .. } | entity::Entity::MetadataLoaded { .. } => {
                Err(CommandEntityError::EntityDataNotLoaded)
            }
            entity::Entity::DataLoaded { ref bevy_entity, ref data, .. } => {
                if data.run_state == EntityRunState::Despawned {
                    return Err(CommandEntityError::EntityNotSpawned);
                }

                entity_commands(commands.entity(bevy_entity.clone()));
        
                Ok(CommandEntitySuccess)
            }
        }
    }
}

// Module Functions
