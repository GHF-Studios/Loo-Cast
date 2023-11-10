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
use crate::AppState;
use crate::game::SimulationState;
use super::entity::{id::*, pos::*};

// External imports
use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
                    .run_if(in_state(SimulationState::Running))
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), ChunkManager::terminate);
    }
}

impl Chunk {
    fn new(id: ChunkID) -> Self {
        Chunk::Registered {
            id,
        }
    }





    pub fn create_chunk_id(&mut self, local_chunk_id: u8) -> Result<ChunkID, String> {
        if local_chunk_id > 99 {
            return Err("Invalid local chunk id.".to_string());
        }

        let mut parent_chunk_id_base10x10;
        match self {
            Chunk::Registered { id, .. } => {
                parent_chunk_id_base10x10 = id.get_global_id_base10x10().clone();
            },
            Chunk::MetadataLoaded { id, .. } => {
                parent_chunk_id_base10x10 = id.get_global_id_base10x10().clone();
            },
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
            },
            Chunk::MetadataLoaded { id, metadata, .. } => {
                parent_chunk_id = id.clone();
                chunk_metadata = metadata;
            },
            Chunk::DataLoaded { id, metadata, .. } => {
                parent_chunk_id = id.clone();
                chunk_metadata = metadata;
            }
        }

        if chunk_metadata.recycled_local_entity_ids.len() != 0 {
            match EntityID::new(parent_chunk_id, chunk_metadata.recycled_local_entity_ids.pop().unwrap()) {
                Ok(entity_id) => {
                    return Ok(entity_id);
                },
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
                },
                Err(e) => Err(format!("Generating a local entity id failed: {}", e)),
            }
        }
    }

    pub fn recycle_entity_id(&mut self, entity_id: EntityID) -> Result<(), String> {
        let mut chunk_metadata; 
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot recycle entity id: No metadata is loaded.".to_string());
            },
            Chunk::MetadataLoaded { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::DataLoaded { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        if chunk_metadata.recycled_local_entity_ids.contains(&entity_id.get_local_id()) {
            return Err("Entity id already recycled.".to_string());
        }

        chunk_metadata.recycled_local_entity_ids.push(entity_id.get_local_id());
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
                },
                Chunk::MetadataLoaded { metadata, .. } => { metadata },
                Chunk::DataLoaded { metadata, .. } => { metadata }
            };

            let gizmo_position: LocalEntityPos = chunk_metadata.get_pos().get_local_pos().clone().into();
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
        let mut registered_chunk = registered_root_chunks.get(&LocalChunkPos::from(root_chunk_id))?.clone();
        drop(registered_root_chunks);
    
        for &local_id in &path {
            let local_id: LocalChunkPos = local_id.into();
            let current_chunk = registered_chunk.lock().ok()?;
            let current_chunk_metadata = match *current_chunk {
                Chunk::Registered { .. } => return None,
                Chunk::MetadataLoaded { metadata, .. } | Chunk::DataLoaded { metadata, .. } => metadata,
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
        let mut registered_chunk = match registered_root_chunks.get(&LocalChunkPos::from(root_chunk_id)) {
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
                Chunk::MetadataLoaded { metadata, .. } | Chunk::DataLoaded { metadata, .. } => metadata,
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

    pub fn request_register(&mut self, id: ChunkID) -> Result<(), String> {
        let mut register_requests = match self.register_requests.lock() {
            Ok(register_requests) => register_requests,
            Err(_) => return Err("Failed to request chunk registration: Register requests mutex poisoned.".to_string()),
        };
        register_requests.push(id);
        return Ok(());
    }

    pub fn request_unregister(&mut self, id: ChunkID) -> Result<(), String> {
        let mut unregister_requests = match self.unregister_requests.lock() {
            Ok(unregister_requests) => unregister_requests,
            Err(_) => return Err("Failed to request chunk unregistration: Unregister requests mutex poisoned.".to_string())
        };
        unregister_requests.push(id);
        return Ok(());
    }

    pub fn request_load_metadata(&mut self, id: ChunkID, metadata: ChunkMetadata) -> Result<(), String> {
        let mut load_metadata_requests = match self.load_metadata_requests.lock() {
            Ok(load_metadata_requests) => load_metadata_requests,
            Err(_) => return Err("Failed to request chunk metadata loading: Load metadata requests mutex poisoned.".to_string())
        };
        load_metadata_requests.push((id, metadata));
        return Ok(());
    }

    pub fn request_unload_metadata(&mut self, id: ChunkID) -> Result<(), String> {
        let mut unload_metadata_requests = match self.unload_metadata_requests.lock() {
            Ok(unload_metadata_requests) => unload_metadata_requests,
            Err(_) => return Err("Failed to request chunk metadata unloading: Unload metadata requests mutex poisoned.".to_string())
        };
        unload_metadata_requests.push(id);
        return Ok(());
    }

    pub fn request_load_data(&mut self, id: ChunkID, data: ChunkData) -> Result<(), String> {
        let mut load_data_requests = match self.load_data_requests.lock() {
            Ok(load_data_requests) => load_data_requests,
            Err(_) => return Err("Failed to request chunk data loading: Load data requests mutex poisoned.".to_string())
        };
        load_data_requests.push((id, data));
        return Ok(());
    }

    pub fn request_unload_data(&mut self, id: ChunkID) -> Result<(), String> {
        let mut unload_data_requests = match self.unload_data_requests.lock() {
            Ok(unload_data_requests) => unload_data_requests,
            Err(_) => return Err("Failed to request chunk data unloading: Unload data requests mutex poisoned.".to_string())
        };
        unload_data_requests.push(id);
        return Ok(());
    }

    pub fn request_spawn(&mut self, id: ChunkID) -> Result<(), String> {
        let mut spawn_requests = match self.spawn_requests.lock() {
            Ok(spawn_requests) => spawn_requests,
            Err(_) => return Err("Failed to request chunk spawning: Spawn requests mutex poisoned.".to_string())
        };
        spawn_requests.push(id);
        return Ok(());
    }

    pub fn request_despawn(&mut self, id: ChunkID) -> Result<(), String> {
        let mut despawn_requests = match self.despawn_requests.lock() {
            Ok(despawn_requests) => despawn_requests,
            Err(_) => return Err("Failed to request chunk despawning: Despawn requests mutex poisoned.".to_string())
        };
        despawn_requests.push(id);
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
            Chunk::Registered { .. } => return Err("Failed to get chunk metadata: Chunk is not loaded.".to_string()),
            Chunk::MetadataLoaded { ref metadata, .. } | Chunk::DataLoaded { ref metadata, .. } => metadata,
        });
    }

    pub fn get_metadata_mut(chunk: &mut Chunk) -> Result<&mut ChunkMetadata, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } => return Err("Failed to get chunk metadata: Chunk is not loaded.".to_string()),
            Chunk::MetadataLoaded { ref mut metadata, .. } | Chunk::DataLoaded { ref mut metadata, .. } => metadata,
        });
    }

    pub fn get_data(chunk: &Chunk) -> Result<&ChunkData, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => return Err("Failed to get chunk data: Chunk is not loaded.".to_string()),
            Chunk::DataLoaded { ref data, .. } => data,
        });
    }

    pub fn get_data_mut(chunk: &mut Chunk) -> Result<&mut ChunkData, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } | Chunk::MetadataLoaded { .. } => return Err("Failed to get chunk data: Chunk is not loaded.".to_string()),
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

    fn handle_register_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_register_requests = chunk_manager.register_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk registration requests: Register requests mutex poisoned.")
        );

        let mut register_requests: Vec<ChunkID> = Vec::new();
        register_requests.append(&mut *chunk_manager_register_requests);

        drop(chunk_manager_register_requests);

        let mut failed_register_requests = Vec::new();
    
        for id in register_requests {
            if chunk_manager.get_registered_chunk(id).is_some() {
                failed_register_requests.push(id);
                continue;
            }

            if id.get_scale_index() == 0 {
                let mut registered_root_chunks = match chunk_manager.registered_root_chunks.lock() {
                    Ok(registered_root_chunks) => registered_root_chunks,
                    Err(_) => {
                        failed_register_requests.push(id);
                        continue;
                    }
                };

                let local_chunk_pos = match id.compute_local_pos() {
                    Ok(local_chunk_pos) => local_chunk_pos,
                    Err(_) => {
                        failed_register_requests.push(id);
                        continue;
                    }
                };

                let chunk = Arc::new(Mutex::new(Chunk::new(id)));

                registered_root_chunks.insert(local_chunk_pos, chunk);

                continue;
            }

            let mut parent_id_base10x10 = id.get_global_id_base10x10().clone();
            parent_id_base10x10.pop();
            let parent_id = match ChunkID::try_from(parent_id_base10x10) {
                Ok(parent_id) => parent_id,
                Err(_) => {
                    failed_register_requests.push(id);
                    continue;
                }
            };

            let parent_chunk = match chunk_manager.get_registered_chunk(parent_id) {
                Some(parent_chunk) => parent_chunk,
                None => {
                    failed_register_requests.push(id);
                    continue;
                }
            };
            let parent_chunk = match parent_chunk.lock() {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    failed_register_requests.push(id);
                    continue;
                }
            };

            let parent_chunk_metadata = match Self::get_metadata(&parent_chunk) {
                Ok(parent_chunk_metadata) => parent_chunk_metadata,
                Err(_) => {
                    failed_register_requests.push(id);
                    continue;
                }
            };

            let mut parent_chunk_child_chunks = match parent_chunk_metadata.child_chunks {
                Some(parent_chunk_child_chunks) => parent_chunk_child_chunks,
                None => {
                    failed_register_requests.push(id);
                    continue;
                }
            };

            let local_chunk_pos = match id.compute_local_pos() {
                Ok(local_chunk_pos) => local_chunk_pos,
                Err(_) => {
                    failed_register_requests.push(id);
                    continue;
                }
            };

            let chunk = Arc::new(Mutex::new(Chunk::new(id)));

            parent_chunk_child_chunks.insert(local_chunk_pos, chunk);
        }
    
        for failed_register_request in &failed_register_requests {
            println!("Failed to register chunk: {:?}", failed_register_request.get_global_id_base10x10());
        }
    }

    fn handle_unregister_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_unregister_requests = chunk_manager.unregister_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk unregistration requests: Unregister requests mutex poisoned.")
        );

        let mut unregister_requests: Vec<ChunkID> = Vec::new();
        unregister_requests.append(&mut *chunk_manager_unregister_requests);

        drop(chunk_manager_unregister_requests);

        let mut failed_unregister_requests = Vec::new();

        for id in unregister_requests {
            let chunk = match chunk_manager.get_registered_chunk(id) {
                Some(chunk) => chunk,
                None => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            if id.get_scale_index() == 0 { 
                let mut registered_root_chunks = match chunk_manager.registered_root_chunks.lock() {
                    Ok(registered_root_chunks) => registered_root_chunks,
                    Err(_) => {
                        failed_unregister_requests.push(id);
                        continue;
                    }
                };

                let local_chunk_pos = match id.compute_local_pos() {
                    Ok(local_chunk_pos) => local_chunk_pos,
                    Err(_) => {
                        failed_unregister_requests.push(id);
                        continue;
                    }
                };

                match registered_root_chunks.remove(&local_chunk_pos) {
                    Some(_) => {},
                    None => {
                        failed_unregister_requests.push(id);
                        continue;
                    }
                };

                continue;
            }

            let chunk = match chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            let parent_chunk_id = match id.compute_parent_id() {
                Ok(parent_chunk_id) => parent_chunk_id,
                Err(_) => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            let parent_chunk = match chunk_manager.get_registered_chunk(id) {
                Some(parent_chunk) => parent_chunk,
                None => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };
            let parent_chunk = match parent_chunk.lock() {
                Ok(parent_chunk) => parent_chunk,
                Err(_) => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            let parent_chunk_metadata = match Self::get_metadata(&parent_chunk) {
                Ok(parent_chunk_metadata) => parent_chunk_metadata,
                Err(_) => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            let mut parent_chunk_child_chunks = match parent_chunk_metadata.child_chunks {
                Some(parent_chunk_child_chunks) => parent_chunk_child_chunks,
                None => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            let local_chunk_pos = match id.compute_local_pos() {
                Ok(local_chunk_pos) => local_chunk_pos,
                Err(_) => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };

            match parent_chunk_child_chunks.remove(&local_chunk_pos) {
                Some(_) => {},
                None => {
                    failed_unregister_requests.push(id);
                    continue;
                }
            };
        }

        for failed_unregister_request in failed_unregister_requests {
            println!("Failed to unregister chunk: {:?}", failed_unregister_request.get_global_id_base10x10());
        }
    }

    fn handle_load_metadata_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_load_metadata_requests = chunk_manager.load_metadata_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk metadata loading requests: Load metadata requests mutex poisoned.")
        );

        let mut load_metadata_requests: Vec<(ChunkID, ChunkMetadata)> = Vec::new();
        load_metadata_requests.append(&mut *chunk_manager_load_metadata_requests);

        drop(chunk_manager_load_metadata_requests);

        let mut failed_load_metadata_requests = Vec::new();

        for (id, metadata) in load_metadata_requests {
            let chunk = match chunk_manager.get_registered_chunk(id) {
                Some(chunk) => chunk,
                None => {
                    failed_load_metadata_requests.push(id);
                    continue;
                }
            };
            let mut chunk = match chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_load_metadata_requests.push(id);
                    continue;
                }
            };

            match *chunk {
                Chunk::Registered { id } => {
                    *chunk = Chunk::MetadataLoaded {
                        id,
                        metadata,
                    };
                },
                Chunk::MetadataLoaded { .. } => {
                    failed_load_metadata_requests.push(id);
                    continue;
                },
                Chunk::DataLoaded { .. } => {
                    failed_load_metadata_requests.push(id);
                    continue;
                }
            }
        }

        for failed_load_metadata_request in failed_load_metadata_requests {
            println!("Failed to load chunk metadata: {:?}", failed_load_metadata_request.get_global_id_base10x10());
        }
    }

    fn handle_unload_metadata_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_unload_metadata_requests = chunk_manager.unload_metadata_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk metadata unloading requests: Unload metadata requests mutex poisoned.")
        );

        let mut unload_metadata_requests: Vec<ChunkID> = Vec::new();
        unload_metadata_requests.append(&mut *chunk_manager_unload_metadata_requests);

        drop(chunk_manager_unload_metadata_requests);

        let mut failed_unload_metadata_requests = Vec::new();

        for id in unload_metadata_requests {
            let chunk = match chunk_manager.get_registered_chunk(id) {
                Some(chunk) => chunk,
                None => {
                    failed_unload_metadata_requests.push(id);
                    continue;
                }
            };
            let mut chunk = match chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_unload_metadata_requests.push(id);
                    continue;
                }
            };

            let chunk_metadata = match Self::get_metadata(&chunk) {
                Ok(chunk_metadata) => chunk_metadata,
                Err(_) => {
                    failed_unload_metadata_requests.push(id);
                    continue;
                }
            };
            if chunk_metadata.child_chunks.as_ref().map_or(false, |c| !c.is_empty()) {
                failed_unload_metadata_requests.push(id);
                continue;
            }
            
            match *chunk {
                Chunk::Registered { .. } => {
                    failed_unload_metadata_requests.push(id);
                    continue;
                },
                Chunk::MetadataLoaded { id, .. } => {
                    *chunk = Chunk::Registered {
                        id,
                    };
                },
                Chunk::DataLoaded { .. } => {
                    failed_unload_metadata_requests.push(id);
                    continue;
                }
            }
        }

        for failed_unload_metadata_request in failed_unload_metadata_requests {
            println!("Failed to unload chunk metadata: {:?}", failed_unload_metadata_request.get_global_id_base10x10());
        }
    }

    fn handle_load_data_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_load_data_requests = chunk_manager.load_data_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk data loading requests: Load data requests mutex poisoned.")
        );

        let mut load_data_requests: Vec<(ChunkID, ChunkData)> = Vec::new();
        load_data_requests.append(&mut *chunk_manager_load_data_requests);

        drop(chunk_manager_load_data_requests);

        let mut failed_load_data_requests = Vec::new();

        for (id, data) in load_data_requests {
            let chunk = match chunk_manager.get_registered_chunk(id) {
                Some(chunk) => chunk,
                None => {
                    failed_load_data_requests.push(id);
                    continue;
                }
            };
            let mut chunk = match chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_load_data_requests.push(id);
                    continue;
                }
            };

            match *chunk {
                Chunk::Registered { .. } => {
                    failed_load_data_requests.push(id);
                    continue;
                },
                Chunk::MetadataLoaded { id, metadata } => {
                    *chunk = Chunk::DataLoaded {
                        id,
                        metadata,
                        data,
                    };
                },
                Chunk::DataLoaded { .. } => {
                    failed_load_data_requests.push(id);
                    continue;
                }
            }
        }

        for failed_load_data_request in failed_load_data_requests {
            println!("Failed to load chunk data: {:?}", failed_load_data_request.get_global_id_base10x10());
        }
    }

    fn handle_unload_data_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_unload_data_requests = chunk_manager.unload_data_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk data unloading requests: Unload data requests mutex poisoned.")
        );

        let mut unload_data_requests: Vec<ChunkID> = Vec::new();
        unload_data_requests.append(&mut *chunk_manager_unload_data_requests);

        drop(chunk_manager_unload_data_requests);

        let mut failed_unload_data_requests = Vec::new();

        for id in unload_data_requests {
            let chunk = match chunk_manager.get_registered_chunk(id) {
                Some(chunk) => chunk,
                None => {
                    failed_unload_data_requests.push(id);
                    continue;
                }
            };
            let mut chunk = match chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_unload_data_requests.push(id);
                    continue;
                }
            };

            let chunk_data = match Self::get_data(&chunk) {
                Ok(chunk_data) => chunk_data,
                Err(_) => {
                    failed_unload_data_requests.push(id);
                    continue;
                }
            };
            if chunk_data.run_state != ChunkRunState::Despawned {
                failed_unload_data_requests.push(id);
                continue;
            }

            match *chunk {
                Chunk::Registered { .. } => {
                    failed_unload_data_requests.push(id);
                    continue;
                },
                Chunk::MetadataLoaded { .. } => {
                    failed_unload_data_requests.push(id);
                    continue;
                },
                Chunk::DataLoaded { id, metadata, .. } => {
                    *chunk = Chunk::MetadataLoaded {
                        id,
                        metadata,
                    };
                }
            }
        }

        for failed_unload_data_request in failed_unload_data_requests {
            println!("Failed to unload chunk data: {:?}", failed_unload_data_request.get_global_id_base10x10());
        }
    }

    fn handle_spawn_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_spawn_requests = chunk_manager.spawn_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk spawning requests: Spawn requests mutex poisoned.")
        );

        let mut spawn_requests: Vec<ChunkID> = Vec::new();
        spawn_requests.append(&mut *chunk_manager_spawn_requests);

        drop(chunk_manager_spawn_requests);

        let mut failed_spawn_requests = Vec::new();

        for id in spawn_requests {
            let chunk_mutex = match chunk_manager.get_registered_chunk(id) {
                Some(chunk_mutex) => chunk_mutex,
                None => {
                    failed_spawn_requests.push(id);
                    continue;
                }
            };
            let mut chunk = match chunk_mutex.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_spawn_requests.push(id);
                    continue;
                }
            };

            let mut chunk_data = match Self::get_data_mut(&mut chunk) {
                Ok(chunk_data) => chunk_data,
                Err(_) => {
                    failed_spawn_requests.push(id);
                    continue;
                }
            };

            match chunk_data.run_state {
                ChunkRunState::Despawned => {
                    chunk_data.run_state = ChunkRunState::Spawned {
                        ecs_entity: commands.spawn(ChunkECSEntity {
                            chunk: chunk_mutex.clone(),
                        }).id(),
                    };
                },
                ChunkRunState::Spawned { .. } => {
                    failed_spawn_requests.push(id);
                    continue;
                }
            } 
        }

        for failed_spawn_request in failed_spawn_requests {
            println!("Failed to spawn chunk: {:?}", failed_spawn_request.get_global_id_base10x10());
        }
    }

    fn handle_despawn_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let mut chunk_manager_despawn_requests = chunk_manager.despawn_requests.lock().unwrap_or_else(|_| 
            panic!("Failed to handle chunk despawning requests: Despawn requests mutex poisoned.")
        );

        let mut despawn_requests: Vec<ChunkID> = Vec::new();
        despawn_requests.append(&mut *chunk_manager_despawn_requests);

        drop(chunk_manager_despawn_requests);

        let mut failed_despawn_requests = Vec::new();

        for id in despawn_requests {
            let chunk = match chunk_manager.get_registered_chunk(id) {
                Some(chunk) => chunk,
                None => {
                    failed_despawn_requests.push(id);
                    continue;
                }
            };
            let mut chunk = match chunk.lock() {
                Ok(chunk) => chunk,
                Err(_) => {
                    failed_despawn_requests.push(id);
                    continue;
                }
            };

            let mut chunk_data = match Self::get_data_mut(&mut chunk) {
                Ok(chunk_data) => chunk_data,
                Err(_) => {
                    failed_despawn_requests.push(id);
                    continue;
                }
            };

            match chunk_data.run_state {
                ChunkRunState::Despawned => {
                    failed_despawn_requests.push(id);
                    continue;
                },
                ChunkRunState::Spawned { ecs_entity } => {
                    commands.entity(ecs_entity).despawn();
                    chunk_data.run_state = ChunkRunState::Despawned;
                }
            } 
        }

        for failed_despawn_request in failed_despawn_requests {
            println!("Failed to despawn chunk: {:?}", failed_despawn_request.get_global_id_base10x10());
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

            let chunk_viewer_local_entity_pos: LocalEntityPos = chunk_viewer_transform.translation.into();
            let chunk_viewer_local_chunk_position: LocalChunkPos = chunk_viewer_local_entity_pos.into();
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
