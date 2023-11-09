// Modules
pub mod data;
pub mod id;
pub mod metadata;
pub mod viewer;

// Local imports
use data::*;
use id::*;
use metadata::*;
use viewer::*;

// Internal imports
use super::entity::*;

// External imports
use std::sync::{Arc, Mutex, RwLock};
use bevy::prelude::*;

// Static variables


// Constant variables
pub const CHUNK_SIZE: u16 = 64;

// Types


// Enums
#[derive(Component)]
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
#[derive(Resource)]
pub struct ChunkManager {
    registered_root_chunks: Arc<Mutex<HashMap<ChunkID, Arc<Mutex<Chunk>>>>>,
    register_requests: Arc<Mutex<Vec<ChunkID>>>,
    unregister_requests: Arc<Mutex<Vec<ChunkID>>>,
    load_metadata_requests: Arc<Mutex<Vec<ChunkID>>>,
    unload_metadata_requests: Arc<Mutex<Vec<ChunkID>>>,
    load_data_requests: Arc<Mutex<Vec<ChunkID>>>,
    unload_data_requests: Arc<Mutex<Vec<ChunkID>>>,
    spawn_requests: Arc<Mutex<Vec<ChunkID>>>,
    despawn_requests: Arc<Mutex<Vec<ChunkID>>>,
}

// Implementations
impl Chunk {
    fn new(id: ChunkID) -> Self {
        Chunk::Registered {
            id: Arc::new(RwLock::new(id)),
        }
    }





    pub fn create_chunk_id(&mut self, local_chunk_id: u8) -> Result<ChunkID, String> {
        if local_chunk_id > 99 {
            return Err("Invalid local chunk id.".to_string());
        }

        let mut parent_chunk_id_base10x10;
        match self {
            Chunk::Registered { id, .. } => {
                parent_chunk_id_base10x10 = id.lock().unwrap().get_global_id_base10x10().clone();
            },
            Chunk::Metadata { id, .. } => {
                parent_chunk_id_base10x10 = id.lock().unwrap().get_global_id_base10x10().clone();
            },
            Chunk::Data { id, .. } => {
                parent_chunk_id_base10x10 = id.lock().unwrap().get_global_id_base10x10().clone();
            }
        }

        if parent_chunk_id_base10x10.len() == 63 {
            return Err("Cannot create chunk id: Max scale index reached.".to_string());
        }

        let chunk_id_base10x10 = parent_chunk_id_base10x10.append(vec![(local_chunk_id / 10, local_chunk_id % 10)]);
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
            Chunk::Metadata { id, metadata, .. } => {
                parent_chunk_id = id.lock().unwrap().clone();
                chunk_metadata = metadata;
            },
            Chunk::Data { id, metadata, .. } => {
                parent_chunk_id = id.lock().unwrap().clone();
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
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        if chunk_metadata.recycled_local_entity_ids.contains(&entity_id.get_local_id()) {
            return Err("Entity id already recycled.".to_string());
        }

        chunk_metadata.recycled_local_entity_ids.push(entity_id.get_local_id());
        Ok(())
    }





    /*
    pub fn register_entity(&mut self, entity_id: EntityID) -> Arc<Mutex<Entity>> {
        let mut chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot register entity: No metadata is loaded.".to_string());
            },
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        match chunk_metadata.registered_entities {
            Some(registered_entities) => {
                if registered_entities.contains_key(&entity_id) {
                    return Err("Cannot register entity: Entity already registered.".to_string());
                }
        
                let entity = Arc::new(Mutex::new(Entity::Registered { id: entity_id.clone(), }));
                registered_entities.insert(entity_id, entity.clone());
                return entity;
            },
            None => {
                return Err("Cannot register entity: Max scale index reached.".to_string());
            }
        }
    }
    
    pub fn get_registered_entity(&mut self, entity_id: EntityID) -> Option<Arc<Mutex<Entity>>> {
        let mut chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot get entity: No metadata is loaded.".to_string());
            },
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        chunk_metadata.registered_entities.get(&entity_id).map(|entity| entity.clone())
    }

    pub fn is_entity_registered(&mut self, entity_id: EntityID) -> bool {
        let mut chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot check if entity is registered: No metadata is loaded.".to_string());
            },
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        chunk_metadata.registered_entities.contains_key(&entity_id)
    }
    */

    fn render_system(mut gizmos: Gizmos, chunk_query: Query<&Chunk>) {
        for chunk in chunk_query.iter() {
            let mut chunk_metadata;
            match self {
                Chunk::Registered { .. } => {
                    continue;
                },
                Chunk::Metadata { metadata, .. } => {
                    chunk_metadata = metadata;
                },
                Chunk::Data { metadata, .. } => {
                    chunk_metadata = metadata;
                }
            }

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
        let mut registered_chunk = registered_root_chunks.get(root_chunk_id.into())?.clone();
        drop(registered_root_chunks);
    
        for &local_id in &path {
            let local_id: LocalChunkPosition = local_id.into();
            let current_chunk = registered_chunk.lock().ok()?;
            let current_chunk_metadata = match current_chunk {
                Chunk::Registered { .. } => return None,
                Chunk::Metadata { metadata, .. } | Chunk::Data { metadata, .. } => metadata,
            };
            registered_chunk = current_chunk_metadata.child_chunks.get(&local_id)?.clone();
        }
    
        Some(registered_chunk)
    }
    
    pub fn is_chunk_registered(&self, id: ChunkID) -> bool {
        let mut path = id.get_global_id_base10x10().clone();
        if path.is_empty() {
            return false;
        }
        let root_chunk_id = path.remove(0);
        let registered_root_chunks = self.registered_root_chunks.lock().unwrap_or_else(|_| return false);
        let mut registered_chunk = registered_root_chunks.get(root_chunk_id.into()).unwrap_or_else(|_| return false);
        drop(registered_root_chunks);
    
        for &local_id in &path {
            let local_id: LocalChunkPosition = local_id.into();
            let current_chunk = registered_chunk.lock().unwrap_or_else(|_| return false);
            let current_chunk_metadata = match current_chunk {
                Chunk::Registered { .. } => return false,
                Chunk::Metadata { metadata, .. } | Chunk::Data { metadata, .. } => metadata,
            };
            registered_chunk = current_chunk_metadata.child_chunks.get(&local_id).unwrap_or_else(|_| return false);
        }
    
        true
    }

    pub fn request_register(&mut self, id: ChunkID) -> Result<(), String> {
        let register_requests = self.register_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk registration: Register requests mutex poisoned.".to_string())
        );
        register_requests.push(id);
    }

    pub fn request_unregister(&mut self, id: ChunkID) -> Result<(), String> {
        let unregister_requests = self.unregister_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk unregistration: Unregister requests mutex poisoned.".to_string())
        );
        unregister_requests.push(id);
    }

    pub fn request_load_metadata(&mut self, id: ChunkID, metadata: ChunkMetadata) -> Result<(), String> {
        let load_metadata_requests = self.load_metadata_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk metadata loading: Load metadata requests mutex poisoned.".to_string())
        );
        load_metadata_requests.push(id);
    }

    pub fn request_unload_metadata(&mut self, id: ChunkID) -> Result<(), String> {
        let unload_metadata_requests = self.unload_metadata_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk metadata unloading: Unload metadata requests mutex poisoned.".to_string())
        );
        unload_metadata_requests.push(id);
    }

    pub fn request_load_data(&mut self, id: ChunkID, data: ChunkData) -> Result<(), String> {
        let load_data_requests = self.load_data_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk data loading: Load data requests mutex poisoned.".to_string())
        );
        load_data_requests.push(id);
    }

    pub fn request_unload_data(&mut self, id: ChunkID) -> Result<(), String> {
        let unload_data_requests = self.unload_data_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk data unloading: Unload data requests mutex poisoned.".to_string())
        );
        unload_data_requests.push(id);
    }

    pub fn request_spawn(&mut self, id: ChunkID) -> Result<(), String> {
        let spawn_requests = self.spawn_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk spawning: Spawn requests mutex poisoned.".to_string())
        );
        spawn_requests.push(id);
    }

    pub fn request_despawn(&mut self, id: ChunkID) -> Result<(), String> {
        let despawn_requests = self.despawn_requests.lock().unwrap_or_else(|_| 
            return Err("Failed to request chunk despawning: Despawn requests mutex poisoned.".to_string())
        );
        despawn_requests.push(id);
    }

    pub fn get_id(chunk: &Chunk) -> Result<&ChunkID, String> {
        return Ok(match *chunk {
            Chunk::Registered { ref id, .. }
            | Chunk::Metadata { ref id, .. }
            | Chunk::Data { ref id, .. } => id,
        });
    }

    pub fn get_metadata(chunk: &Chunk) -> Result<&ChunkMetadata, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } => return Err("Failed to get chunk metadata: Chunk is not loaded.".to_string()),
            Chunk::Metadata { ref metadata, .. } | Chunk::Data { ref metadata, .. } => metadata,
        });
    }

    pub fn get_metadata_mut(chunk: &mut Chunk) -> Result<&mut ChunkMetadata, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } => return Err("Failed to get chunk metadata: Chunk is not loaded.".to_string()),
            Chunk::Metadata { ref mut metadata, .. } | Chunk::Data { ref mut metadata, .. } => metadata,
        });
    }

    pub fn get_data(chunk: &Chunk) -> Result<&ChunkData, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } | Chunk::Metadata { .. } => return Err("Failed to get chunk data: Chunk is not loaded.".to_string()),
            Chunk::Data { ref data, .. } => data,
        });
    }

    pub fn get_data_mut(chunk: &mut Chunk) -> Result<&mut ChunkData, String> {
        return Ok(match *chunk {
            Chunk::Registered { .. } | Chunk::Metadata { .. } => return Err("Failed to get chunk data: Chunk is not loaded.".to_string()),
            Chunk::Data { ref mut data, .. } => data,
        });
    }

    pub fn get_load_state(chunk: &Chunk) -> Result<ChunkLoadState, String> {
        return match *chunk {
            Chunk::Registered { .. } => Ok(ChunkLoadState::Registered),
            Chunk::Metadata { .. } => Ok(ChunkLoadState::MetadataLoaded),
            Chunk::Data { .. } => Ok(ChunkLoadState::DataLoaded),
        };
    }

    fn handle_register_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let register_requests: Vec<ChunkPos> = Vec::new();
        register_requests.append(chunk_manager.register_requests);
        let mut failed_register_requests = Vec::new();
    
        for id in register_requests {
            fn fail_request() {
                failed_register_requests.push(id);
                continue;
            }

            if chunk_manager.get_registered_chunk(id).is_some() { fail_request(); }

            let mut parent_id_base10x10 = id.get_global_id_base10x10().clone();
            parent_chunk_id_base10x10.pop();
            let parent_id = ChunkID::try_from(parent_id_base10x10).unwrap_or_else(|_| fail_request());

            let parent_chunk = chunk_manager.get_registered_chunk(parent_id).unwrap_or_else(|_| fail_request());
            let parent_chunk = parent_chunk.lock().unwrap_or_else(|_| fail_request());

            let parent_chunk_metadata = chunk_manager.get_metadata(&parent_chunk).unwrap_or_else(|_| fail_request());

            let mut parent_chunk_child_chunks = parent_chunk_metadata.child_chunks.unwrap_or_else(|_| fail_request());

            let local_chunk_pos = id.compute_local_pos();
            let chunk = Arc::new(Mutex::new(Chunk::new(id)));
            parent_chunk_child_chunks.insert(local_chunk_pos, chunk);
        }
    
        for failed_register_request in &failed_register_requests {
            println!("Failed to register chunk: {}", failed_register_request);
        }
    }

    fn handle_unregister_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let unregister_requests: Vec<ChunkPos> = Vec::new();
        unregister_requests.append(chunk_manager.unregister_requests);
        let mut failed_unregister_requests = Vec::new();

        for id in unregister_requests {
            fn fail_request() {
                failed_unregister_requests.push(id);
                continue;
            }
        }

        for failed_unregister_request in failed_unregister_requests {
            println!("Failed to unregister chunk: {}", failed_unregister_request);
        }
    }

    fn handle_load_metadata_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let load_metadata_requests: Vec<ChunkPos> = Vec::new();
        load_metadata_requests.append(chunk_manager.load_metadata_requests);
        let mut failed_load_metadata_requests = Vec::new();

        for id in load_metadata_requests {
            fn fail_request() {
                failed_load_metadata_requests.push(id);
                continue;
            }
        }

        for failed_load_metadata_request in failed_load_metadata_requests {
            println!("Failed to load chunk metadata: {}", failed_load_metadata_request);
        }
    }

    fn handle_unload_metadata_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let unload_metadata_requests: Vec<ChunkPos> = Vec::new();
        unload_metadata_requests.append(chunk_manager.unload_metadata_requests);
        let mut failed_unload_metadata_requests = Vec::new();

        for id in unload_metadata_requests {
            fn fail_request() {
                failed_unload_metadata_requests.push(id);
                continue;
            }
        }

        for failed_unload_metadata_request in failed_unload_metadata_requests {
            println!("Failed to unload chunk metadata: {}", failed_unload_metadata_request);
        }
    }

    fn handle_load_data_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let load_data_requests: Vec<ChunkPos> = Vec::new();
        load_data_requests.append(chunk_manager.load_data_requests);
        let mut failed_load_data_requests = Vec::new();

        for id in load_data_requests {
            fn fail_request() {
                failed_load_data_requests.push(id);
                continue;
            }
        }

        for failed_load_data_request in failed_load_data_requests {
            println!("Failed to load chunk data: {}", failed_load_data_request);
        }
    }

    fn handle_unload_data_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let unload_data_requests: Vec<ChunkPos> = Vec::new();
        unload_data_requests.append(chunk_manager.unload_data_requests);
        let mut failed_unload_data_requests = Vec::new();

        for id in unload_data_requests {
            fn fail_request() {
                failed_unload_data_requests.push(id);
                continue;
            }
        }

        for failed_unload_data_request in failed_unload_data_requests {
            println!("Failed to unload chunk data: {}", failed_unload_data_request);
        }
    }

    fn handle_spawn_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let spawn_requests: Vec<ChunkPos> = Vec::new();
        spawn_requests.append(chunk_manager.spawn_requests);
        let mut failed_spawn_requests = Vec::new();

        for id in spawn_requests {
            fn fail_request() {
                failed_spawn_requests.push(id);
                continue;
            }
        }

        for failed_spawn_request in failed_spawn_requests {
            println!("Failed to spawn chunk: {}", failed_spawn_request);
        }
    }

    fn handle_despawn_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let despawn_requests: Vec<ChunkPos> = Vec::new();
        despawn_requests.append(chunk_manager.despawn_requests);
        let mut failed_despawn_requests = Vec::new();

        for id in despawn_requests {
            fn fail_request() {
                failed_despawn_requests.push(id);
                continue;
            }
        }

        for failed_despawn_request in failed_despawn_requests {
            println!("Failed to despawn chunk: {}", failed_despawn_request);
        }
    }
}

// Module Functions
