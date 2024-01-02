// Modules
pub mod id;

// Local imports

// Internal imports
use crate::engine::system::game::SimulationState;
use crate::engine::system::universe::chunk::data::*;
use crate::engine::system::universe::chunk::id::*;
use crate::engine::system::universe::chunk::metadata::*;
use crate::engine::system::universe::chunk::*;
use crate::engine::system::universe::entity::data::*;
use crate::engine::system::universe::entity::id::*;
use crate::engine::system::universe::entity::metadata::*;
use crate::engine::system::universe::entity::*;
use crate::engine::system::universe::*;
use crate::engine::system::AppState;

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
    pub registered_root_chunks:
        HashMap<LocalChunkID, Arc<Mutex<Chunk>>>,
    pub operation_requests: Arc<Mutex<Vec<OperationRequest>>>,
    pub chunk_entity_info_hierarchy: ChunkEntityInfoHierarchy,
}

pub(in crate::engine::system::universe) struct ChunkEntityInfoHierarchy {
    root_chunks: HashMap<LocalChunkID, Arc<Mutex<ChunkInfo>>>,
}

#[derive(Clone)]
pub(in crate::engine::system::universe) struct ChunkInfo {
    parent_chunk_info_mutex: Option<Arc<Mutex<ChunkInfo>>>,
    local_chunk_id: LocalChunkID,
    chunk_id: ChunkID,
    chunk_mutex: Arc<Mutex<Chunk>>,
    child_chunks: Option<HashMap<LocalChunkID, Arc<Mutex<ChunkInfo>>>>,
    child_entities: HashMap<LocalEntityID, Arc<Mutex<EntityInfo>>>,
}

#[derive(Clone)]
pub(in crate::engine::system::universe) struct EntityInfo {
    parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>,
    local_entity_id: LocalEntityID,
    entity_id: EntityID,
    entity_mutex: Arc<Mutex<entity::Entity>>,
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
    pub(in crate::engine::system::universe) fn generate_entity_id(parent_chunk: &mut Chunk) -> Result<EntityID, String> {
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

    pub(in crate::engine::system::universe) fn recycle_entity_id(parent_chunk: &mut Chunk, entity_id: EntityID) -> Result<(), String> {
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

    pub(in crate::engine::system::universe) fn get_registered_chunk(&self, chunk_id: &ChunkID,) -> Option<Arc<Mutex<Chunk>>> {
        let chunk_info_mutex = match self.chunk_entity_info_hierarchy.get_chunk_info(chunk_id) {
            Some(chunk_info_mutex) => chunk_info_mutex,
            None => {
                return None;
            }
        };

        let chunk_info = match chunk_info_mutex.lock() {
            Ok(chunk_info) => chunk_info,
            Err(_) => {
                panic!("Failed to get registered chunk: Chunk info mutex poisoned.");
            }
        };

        Some(chunk_info.chunk_mutex.clone())
    }

    pub(in crate::engine::system::universe) fn is_chunk_registered(&self, chunk_id: &ChunkID) -> bool {
        self.chunk_entity_info_hierarchy.is_chunk_info_registered(chunk_id)
    }

    pub(in crate::engine::system::universe) fn get_registered_entity(&self, entity_id: &EntityID,) -> Option<Arc<Mutex<entity::Entity>>> {
        let entity_info_mutex = match self.chunk_entity_info_hierarchy.get_entity_info(entity_id) {
            Some(entity_info_mutex) => entity_info_mutex,
            None => {
                return None;
            }
        };

        let entity_info = match entity_info_mutex.lock() {
            Ok(entity_info) => entity_info,
            Err(_) => {
                panic!("Failed to get registered entity: Entity info mutex poisoned.");
            }
        };

        Some(entity_info.entity_mutex.clone())
    }

    pub(in crate::engine::system::universe) fn is_entity_registered(&self, entity_id: &EntityID) -> bool {
        self.chunk_entity_info_hierarchy.is_entity_info_registered(entity_id)
    }

    pub(in crate::engine::system::universe) fn send_chunk_operation_request(
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

    pub(in crate::engine::system::universe) fn send_entity_operation_request(
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
                ChunkOperation::RegisterRoot { 
                    local_chunk_id, 
                    success_callback, 
                    failure_callback 
                } => {
                    match self.register_root_chunk(commands, local_chunk_id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::Register {
                    parent_chunk,
                    local_chunk_id,
                    success_callback,
                    failure_callback,
                } => {
                    let mut parent_chunk = match parent_chunk.lock() {
                        Ok(parent_chunk) => parent_chunk,
                        Err(_) => {
                            panic!("Failed to register chunk: Parent chunk mutex poisoned.");
                        }
                    };

                    match self.register_chunk(commands, &mut *parent_chunk, local_chunk_id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::UnregisterRoot { 
                    chunk, 
                    success_callback, 
                    failure_callback 
                } => {
                    let chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to unregister chunk: Chunk mutex poisoned.");
                        }
                    };

                    match self.unregister_root_chunk(commands, &*chunk) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::Unregister {
                    parent_chunk,
                    chunk,
                    success_callback,
                    failure_callback,
                } => {
                    let mut parent_chunk = match parent_chunk.lock() {
                        Ok(parent_chunk) => parent_chunk,
                        Err(_) => {
                            panic!("Failed to unregister chunk: Parent chunk mutex poisoned.");
                        }
                    };

                    let chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to unregister chunk: Chunk mutex poisoned.");
                        }
                    };

                    match self.unregister_chunk(commands, &mut *parent_chunk, &*chunk) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::LoadMetadata {
                    chunk,
                    metadata,
                    success_callback,
                    failure_callback,
                } => {
                    let mut chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to load chunk metadata: Chunk mutex poisoned.");
                        }
                    };

                    match Self::load_chunk_metadata(&mut *chunk, metadata) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::UnloadMetadata {
                    chunk,
                    success_callback,
                    failure_callback,
                } => {
                    let mut chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to unload chunk metadata: Chunk mutex poisoned.");
                        }
                    };

                    match Self::unload_chunk_metadata(&mut *chunk) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::LoadData {
                    chunk,
                    data,
                    success_callback,
                    failure_callback,
                } => {
                    let mut chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to load chunk data: Chunk mutex poisoned.");
                        }
                    };

                    match Self::load_chunk_data(&mut *chunk, data) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::UnloadData {
                    chunk,
                    success_callback,
                    failure_callback,
                } => {
                    let mut chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to unload chunk data: Chunk mutex poisoned.");
                        }
                    };

                    match Self::unload_chunk_data(&mut *chunk) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::Spawn {
                    parent_chunk,
                    chunk,
                    success_callback,
                    failure_callback,
                } => {
                    let parent_chunk = match parent_chunk.lock() {
                        Ok(parent_chunk) => parent_chunk,
                        Err(_) => {
                            panic!("Failed to spawn chunk: Parent chunk mutex poisoned.");
                        }
                    };

                    let mut chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to spawn chunk: Chunk mutex poisoned.");
                        }
                    };

                    match Self::spawn_chunk(commands, &*parent_chunk, &mut *chunk) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                ChunkOperation::Despawn {
                    chunk,
                    success_callback,
                    failure_callback,
                } => {
                    let mut chunk = match chunk.lock() {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            panic!("Failed to despawn chunk: Chunk mutex poisoned.");
                        }
                    };

                    match Self::despawn_chunk(commands, &mut *chunk) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
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
                    parent_chunk,
                    local_entity_id,
                    success_callback,
                    failure_callback,
                } => {
                    let mut parent_chunk = match parent_chunk.lock() {
                        Ok(parent_chunk) => parent_chunk,
                        Err(_) => {
                            panic!("Failed to register entity: Parent chunk mutex poisoned.");
                        }
                    };

                    match Self::register_entity(commands, &mut *parent_chunk, local_entity_id) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::Unregister {
                    parent_chunk,
                    entity,
                    success_callback,
                    failure_callback,
                } => {
                    let mut parent_chunk = match parent_chunk.lock() {
                        Ok(parent_chunk) => parent_chunk,
                        Err(_) => {
                            panic!("Failed to unregister entity: Parent chunk mutex poisoned.");
                        }
                    };

                    let entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to unregister entity: Entity mutex poisoned.");
                        }
                    };
                    
                    match Self::unregister_entity(commands, &mut *parent_chunk, &*entity) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::LoadMetadata {
                    entity,
                    metadata,
                    success_callback,
                    failure_callback,
                } => {
                    let mut entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to load entity metadata: Entity mutex poisoned.");
                        }
                    };

                    match Self::load_entity_metadata(&mut *entity, metadata) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::UnloadMetadata {
                    entity,
                    success_callback,
                    failure_callback,
                } => {
                    let mut entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to unload entity metadata: Entity mutex poisoned.");
                        }
                    };

                    match Self::unload_entity_metadata(&mut *entity) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::LoadData {
                    entity,
                    data,
                    success_callback,
                    failure_callback,
                } => {
                    let mut entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to load entity data: Entity mutex poisoned.");
                        }
                    };

                    match Self::load_entity_data(&mut *entity, data) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::UnloadData {
                    entity,
                    success_callback,
                    failure_callback,
                } => {
                    let mut entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to unload entity data: Entity mutex poisoned.");
                        }
                    };

                    match Self::unload_entity_data(&mut *entity) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::Spawn {
                    parent_chunk,
                    entity,
                    success_callback,
                    failure_callback,
                } => {
                    let parent_chunk = match parent_chunk.lock() {
                        Ok(parent_chunk) => parent_chunk,
                        Err(_) => {
                            panic!("Failed to spawn entity: Parent chunk mutex poisoned.");
                        }
                    };

                    let mut entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to spawn entity: Entity mutex poisoned.");
                        }
                    };

                    match Self::spawn_entity(commands, &*parent_chunk, &mut *entity) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::Despawn {
                    entity,
                    success_callback,
                    failure_callback,
                } => {
                    let mut entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to despawn entity: Entity mutex poisoned.");
                        }
                    };

                    match Self::despawn_entity(commands, &mut *entity) {
                        Ok(success) => {
                            success_callback(success);
                        }
                        Err(error) => {
                            failure_callback(error);
                        }
                    }
                },
                EntityOperation::Command { 
                    entity_commands, 
                    entity, 
                    success_callback, 
                    failure_callback 
                } => {
                    let entity = match entity.lock() {
                        Ok(entity) => entity,
                        Err(_) => {
                            panic!("Failed to command entity: Entity mutex poisoned.");
                        }
                    };

                    match Self::command_entity(commands, entity_commands, &*entity) {
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
    }

    fn register_root_chunk(
        &mut self,
        commands: &mut Commands,
        local_chunk_id: LocalChunkID,
    ) -> Result<RegisterRootChunkSuccess, RegisterRootChunkError> {
        if self.registered_root_chunks.contains_key(&local_chunk_id) {
            return Err(RegisterRootChunkError::ChunkAlreadyRegistered);
        }

        let chunk_id = ChunkID::new_root(local_chunk_id);

        let chunk_bevy_entity = commands.spawn(()).id();

        let chunk = Arc::new(Mutex::new(Chunk::new(chunk_id.clone(), chunk_bevy_entity)));

        commands.entity(chunk_bevy_entity).insert(ChunkBevyComponent {
            chunk: chunk.clone(),
        });

        self.registered_root_chunks.insert(local_chunk_id, chunk);
        self.chunk_entity_info_hierarchy.insert_chunk_info(None, local_chunk_id, chunk);

        return Ok(RegisterRootChunkSuccess);
    }

    fn register_chunk(
        &mut self,
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

        let chunk_id = match ChunkID::new(parent_chunk_id.clone(), local_chunk_id) {
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
        self.chunk_entity_info_hierarchy.insert_chunk_info(Some(parent_chunk_id), local_chunk_id, chunk);
        

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

        let local_chunk_id = chunk_id.get_local_chunk_id();

        if !self.registered_root_chunks.contains_key(&local_chunk_id) {
            return Err(UnregisterRootChunkError::ChunkAlreadyUnregistered);
        }

        self.registered_root_chunks.remove(&local_chunk_id);
        self.chunk_entity_info_hierarchy.remove_chunk_info(&chunk_id);

        commands.entity(chunk_bevy_entity).despawn();

        return Ok(UnregisterRootChunkSuccess);
    }

    fn unregister_chunk(
        &mut self,
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

        let local_chunk_id = chunk_id.get_local_chunk_id();

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
        self.chunk_entity_info_hierarchy.remove_chunk_info(&chunk_id);

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
        parent_chunk: &Chunk,
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

        let local_chunk_id = chunk_id.get_local_chunk_id();

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
        entity: &crate::engine::system::universe::entity::Entity,
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
        entity: &mut crate::engine::system::universe::entity::Entity,
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
        entity: &mut crate::engine::system::universe::entity::Entity,
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
        entity: &mut crate::engine::system::universe::entity::Entity,
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
        entity: &mut crate::engine::system::universe::entity::Entity,
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
        parent_chunk: &Chunk,
        entity: &mut crate::engine::system::universe::entity::Entity,
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
        entity: &mut crate::engine::system::universe::entity::Entity,
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
        entity: &crate::engine::system::universe::entity::Entity,
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

impl ChunkEntityInfoHierarchy {
    pub(in crate::engine::system::universe) fn new() -> Self {
        Self {
            root_chunks: HashMap::new(),
        }
    }

    pub(in crate::engine::system::universe) fn get_chunk_info(&self, chunk_id: &ChunkID) -> Option<Arc<Mutex<ChunkInfo>>> {
        if let Some(parent_chunk_id) = chunk_id.get_parent_chunk_id() {
            let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
                None => {
                    return None;
                }
            };

            let parent_chunk_info = match parent_chunk_info_mutex.lock() {
                Ok(parent_chunk_info) => parent_chunk_info,
                Err(_) => {
                    panic!("Failed to get chunk info: Parent chunk info mutex poisoned.");
                }
            };

            let child_chunk_infos = match parent_chunk_info.child_chunks {
                Some(ref child_chunk_infos) => child_chunk_infos,
                None => {
                    return None;
                }
            };

            child_chunk_infos.get(&chunk_id.get_local_chunk_id()).cloned()
        } else {
            self.root_chunks.get(&chunk_id.get_local_chunk_id()).cloned()
        }
    }

    pub(in crate::engine::system::universe) fn is_chunk_info_registered(&self, chunk_id: &ChunkID) -> bool {
        if let Some(parent_chunk_id) = chunk_id.get_parent_chunk_id() {
            let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
                None => {
                    return false;
                }
            };

            let parent_chunk_info = match parent_chunk_info_mutex.lock() {
                Ok(parent_chunk_info) => parent_chunk_info,
                Err(_) => {
                    panic!("Failed to check if chunk info is registered: Parent chunk info mutex poisoned.");
                }
            };

            let child_chunk_infos = match parent_chunk_info.child_chunks {
                Some(ref child_chunk_infos) => child_chunk_infos,
                None => {
                    return false;
                }
            };

            child_chunk_infos.contains_key(&chunk_id.get_local_chunk_id())
        } else {
            self.root_chunks.contains_key(&chunk_id.get_local_chunk_id())
        }
    }

    pub(in crate::engine::system::universe) fn insert_chunk_info(&self, parent_chunk_id: Option<&ChunkID>, local_chunk_id: LocalChunkID, chunk_mutex: Arc<Mutex<Chunk>>) -> Result<(), String> {
        match parent_chunk_id {
            Some(parent_chunk_id) => {
                let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                    Some(parent_chunk_info) => parent_chunk_info,
                    None => {
                        return Err(format!("Failed to insert chunk info: Parent chunk info not found."));
                    }
                };
        
                let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
                    Ok(parent_chunk_info) => parent_chunk_info,
                    Err(_) => {
                        panic!("Failed to insert chunk info: Parent chunk info mutex poisoned.");
                    }
                };
        
                let child_chunks = match parent_chunk_info.child_chunks {
                    Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
                    None => {
                        return Err(format!("Failed to insert chunk info: Parent chunk info not allowed to have child chunk infos."));
                    }
                };
        
                if child_chunks.contains_key(&local_chunk_id) {
                    return Err(format!("Failed to insert chunk info: Chunk info already registered."));
                }
        
                let chunk_info = match ChunkInfo::new(parent_chunk_info_mutex, local_chunk_id, chunk_mutex) {
                    Ok(chunk_info) => chunk_info,
                    Err(error) => {
                        return Err(format!("Failed to insert chunk info: {}", error));
                    }
                };
        
                child_chunks.insert(local_chunk_id, Arc::new(Mutex::new(chunk_info)));
        
                Ok(())
            },
            None => {
                if self.root_chunks.contains_key(&local_chunk_id) {
                    return Err(format!("Failed to insert chunk info: Chunk info already registered."));
                }
        
                let chunk_info = ChunkInfo::new_root(local_chunk_id, chunk_mutex);
        
                self.root_chunks.insert(local_chunk_id, Arc::new(Mutex::new(chunk_info)));
        
                Ok(())
            }
        }
    }

    pub(in crate::engine::system::universe) fn remove_chunk_info(&self, chunk_id: &ChunkID) -> Result<(), String> {
        match chunk_id.get_parent_chunk_id() {
            Some(parent_chunk_id) => {
                let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                    Some(parent_chunk_info) => parent_chunk_info,
                    None => {
                        return Err(format!("Failed to remove chunk info: Parent chunk info not found."));
                    }
                };
        
                let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
                    Ok(parent_chunk_info) => parent_chunk_info,
                    Err(_) => {
                        panic!("Failed to remove chunk info: Parent chunk info mutex poisoned.");
                    }
                };
        
                let child_chunks = match parent_chunk_info.child_chunks {
                    Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
                    None => {
                        return Err(format!("Failed to remove chunk info: Parent chunk info not allowed to have child chunk infos."));
                    }
                };

                match child_chunks.remove(&chunk_id.get_local_chunk_id()) {
                    Some(_) => {
                        Ok(())
                    }
                    None => {
                        Err(format!("Failed to remove chunk info: Chunk info not registered."))
                    }
                }
            },
            None => {
                if !self.root_chunks.contains_key(&chunk_id.get_local_chunk_id()) {
                    return Err(format!("Failed to remove chunk info: Chunk info not registered."));
                }

                match self.root_chunks.remove(&chunk_id.get_local_chunk_id()) {
                    Some(_) => {
                        Ok(())
                    }
                    None => {
                        Err(format!("Failed to remove chunk info: Chunk info not registered."))
                    }
                }
            }
        }
    }

    pub(in crate::engine::system::universe) fn get_entity_info(&self, entity_id: &EntityID) -> Option<Arc<Mutex<EntityInfo>>> {
        let parent_chunk_info_mutex = match self.get_chunk_info(entity_id.get_parent_chunk_id()) {
            Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
            None => {
                return None;
            }
        };

        let parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to get entity info: Parent chunk info mutex poisoned.");
            }
        };

        parent_chunk_info.child_entities.get(&entity_id.get_local_entity_id()).cloned()
    }

    pub(in crate::engine::system::universe) fn is_entity_info_registered(&self, entity_id: &EntityID) -> bool {
        let parent_chunk_info_mutex = match self.get_chunk_info(entity_id.get_parent_chunk_id()) {
            Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
            None => {
                return false;
            }
        };

        let parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to check if entity info is registered: Parent chunk info mutex poisoned.");
            }
        };

        parent_chunk_info.child_entities.contains_key(&entity_id.get_local_entity_id())
    }

    pub(in crate::engine::system::universe) fn insert_entity_info(&self, parent_chunk_id: &ChunkID, local_entity_id: LocalEntityID, entity_mutex: Arc<Mutex<entity::Entity>>) -> Result<(), String> {
        let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
            Some(parent_chunk_info) => parent_chunk_info,
            None => {
                return Err(format!("Failed to insert entity info: Parent chunk info not found."));
            }
        };

        let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to insert entity info: Parent chunk info mutex poisoned.");
            }
        };

        if parent_chunk_info.child_entities.contains_key(&local_entity_id) {
            return Err(format!("Failed to insert entity info: Entity info already registered."));
        }

        let entity_info = EntityInfo::new(parent_chunk_info_mutex, local_entity_id, entity_mutex);

        parent_chunk_info.child_entities.insert(local_entity_id, Arc::new(Mutex::new(entity_info)));

        Ok(())
    }

    pub(in crate::engine::system::universe) fn remove_entity_info(&self, entity_id: &EntityID) -> Result<(), String> {
        let parent_chunk_info_mutex = match self.get_chunk_info(entity_id.get_parent_chunk_id()) {
            Some(parent_chunk_info) => parent_chunk_info,
            None => {
                return Err(format!("Failed to remove entity info: Parent chunk info not found."));
            }
        };

        let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to remove entity info: Parent chunk info mutex poisoned.");
            }
        };

        match parent_chunk_info.child_entities.remove(&entity_id.get_local_entity_id()) {
            Some(_) => {
                Ok(())
            }
            None => {
                Err(format!("Failed to remove entity info: Entity info not registered."))
            }
        }
    }
}

impl ChunkInfo {
    fn new_root(local_chunk_id: LocalChunkID, chunk_mutex: Arc<Mutex<Chunk>>) -> Self {
        Self {
            parent_chunk_info_mutex: None,
            local_chunk_id,
            chunk_id: ChunkID::new_root(local_chunk_id),
            chunk_mutex,
            child_chunks: Some(HashMap::new()),
            child_entities: HashMap::new(),
        }
    }

    fn new(parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>, local_chunk_id: LocalChunkID, chunk_mutex: Arc<Mutex<Chunk>>) -> Result<Self, String> {
        let parent_chunk_id = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info.chunk_id.clone(),
            Err(_) => {
                panic!("Failed to create chunk info: Parent chunk info mutex poisoned.");
            }
        };

        let chunk_id = match ChunkID::new(parent_chunk_id, local_chunk_id) {
            Ok(chunk_id) => chunk_id,
            Err(error) => {
                return Err(format!("Failed to create chunk info: {}", error));
            }
        };
        
        Ok(Self {
            parent_chunk_info_mutex: Some(parent_chunk_info_mutex),
            local_chunk_id,
            chunk_id,
            chunk_mutex,
            child_chunks: Some(HashMap::new()),
            child_entities: HashMap::new(),
        })
    }

    fn new_leaf(parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>, local_chunk_id: LocalChunkID, chunk_mutex: Arc<Mutex<Chunk>>) -> Result<Self, String> {
        let parent_chunk_id = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info.chunk_id.clone(),
            Err(_) => {
                panic!("Failed to create chunk info: Parent chunk info mutex poisoned.");
            }
        };

        let chunk_id = match ChunkID::new(parent_chunk_id, local_chunk_id) {
            Ok(chunk_id) => chunk_id,
            Err(error) => {
                return Err(format!("Failed to create chunk info: {}", error));
            }
        };
        
        Ok(Self {
            parent_chunk_info_mutex: Some(parent_chunk_info_mutex),
            local_chunk_id,
            chunk_id,
            chunk_mutex,
            child_chunks: None,
            child_entities: HashMap::new(),
        })
    }
}

impl EntityInfo {
    fn new(parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>, local_entity_id: LocalEntityID, entity_mutex: Arc<Mutex<entity::Entity>>) -> Self {
        let parent_chunk_id = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info.chunk_id.clone(),
            Err(_) => {
                panic!("Failed to create entity info: Parent chunk info mutex poisoned.");
            }
        };

        Self {
            parent_chunk_info_mutex,
            local_entity_id,
            entity_id: EntityID::new(parent_chunk_id, local_entity_id),
            entity_mutex,
        }
    }
}

// Module Functions
