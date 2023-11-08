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
        id: Arc<RwLock<ChunkID>>,
    },
    MetadataLoaded {
        id: Arc<RwLock<ChunkID>>,
        metadata: Arc<Mutex<ChunkMetadata>>,
    },
    DataLoaded {
        id: Arc<RwLock<ChunkID>>,
        metadata: Arc<Mutex<ChunkMetadata>>,
        data: Arc<Mutex<ChunkData>>,
    },
}

pub enum ChunkLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

// Structs
#[derive(Resource)]
pub struct ChunkManager {
    registered_root_chunks: HashMap<ChunkID, Arc<Mutex<Chunk>>>,
    register_requests: Vec<ChunkID>,
    unregister_requests: Vec<ChunkID>,
    load_metadata_requests: Vec<ChunkID>,
    unload_metadata_requests: Vec<ChunkID>,
    load_data_requests: Vec<ChunkID>,
    unload_data_requests: Vec<ChunkID>,
    spawn_requests: Vec<ChunkID>,
    despawn_requests: Vec<ChunkID>,
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
    pub fn register_chunk(&mut self, chunk_id: ChunkID) -> Arc<Mutex<Chunk>> {
        let mut chunk_metadata; 
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot register chunk: No metadata is loaded.".to_string());
            },
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        match chunk_metadata.child_chunks {
            Some(child_chunks) => {
                if child_chunks.contains_key(&chunk_id) {
                    return Err("Cannot register chunk: Chunk already registered.".to_string());
                }
        
                let chunk = Arc::new(Mutex::new(Chunk::Registered { id: chunk_id.clone(), }));
                child_chunks.insert(chunk_id, chunk.clone());
                return chunk;
            },
            None => {
                return Err("Cannot register chunk: Max scale index reached.".to_string());
            }
        }
    }

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

    pub fn get_registered_chunk(&mut self, chunk_id: ChunkID) -> Result<Option<Arc<Mutex<Chunk>>>, String> {
        let mut chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot get chunk: No metadata is loaded.".to_string());
            },
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        match chunk_metadata.child_chunks {
            Some(child_chunks) => {
                return Ok(child_chunks.get(&chunk_id).map(|chunk| chunk.clone()));
            },
            None => {
                return Err("Cannot get chunk: Max scale index reached.".to_string());
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

    pub fn is_chunk_registered(&mut self, chunk_id: ChunkID) -> Result<bool, String> {
        let mut chunk_metadata;
        match self {
            Chunk::Registered { .. } => {
                return Err("Cannot check if chunk is registered: No metadata is loaded.".to_string());
            },
            Chunk::Metadata { metadata, .. } => {
                chunk_metadata = metadata;
            },
            Chunk::Data { metadata, .. } => {
                chunk_metadata = metadata;
            }
        }

        match chunk_metadata.child_chunks {
            Some(child_chunks) => {
                Ok(child_chunks.contains_key(&chunk_id))
            },
            None => {
                Err("Cannot check if chunk is registered: Max scale index reached.".to_string())
            }
        }
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




    
    pub fn get_load_state(&self) -> ChunkLoadState {
        match self {
            Chunk::Registered { .. } => ChunkLoadState::Registered,
            Chunk::MetadataLoaded { .. } => ChunkLoadState::MetadataLoaded,
            Chunk::DataLoaded { .. } => ChunkLoadState::DataLoaded,
        }
    }

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
            registered_root_chunks: HashMap::new(),
            spawn_requests: Vec::new(),
            despawn_requests: Vec::new(),
        };

        commands.insert_resource(chunk_manager);
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<Self>();
    }

    pub fn get(&self, id: ChunkID) -> Option<Arc<Mutex<Chunk>>> {
        let id_base10x10 = id.get_global_id_base10x10().clone();
        let scale_index = id.get_scale_index();

        if scale_index == 0 {
            return self.registered_root_chunks.get(&id).map(|chunk| chunk.clone());
        }
    }

    pub fn contains(&self, id: ChunkID) -> bool {
        let id_base10x10 = id.get_global_id_base10x10().clone();
        let scale_index = id.get_scale_index();
    }

    pub fn request_register(&mut self, id: ChunkID) {
        self.register_requests.push(id);
    }

    pub fn request_unregister(&mut self, id: ChunkID) {
        self.unregister_requests.push(id);
    }

    pub fn request_load_metadata(&mut self, id: ChunkID, metadata: ChunkMetadata) {
        self.load_metadata_requests.push(id);
    }

    pub fn request_unload_metadata(&mut self, id: ChunkID) {
        self.unload_metadata_requests.push(id);
    }

    pub fn request_load_data(&mut self, id: ChunkID, data: ChunkData) {
        self.load_data_requests.push(id);
    }

    pub fn request_unload_data(&mut self, id: ChunkID) {
        self.unload_data_requests.push(id);
    }

    pub fn request_spawn(&mut self, id: ChunkID) {
        self.spawn_requests.push(id);
    }

    pub fn request_despawn(&mut self, id: ChunkID) {
        self.despawn_requests.push(id);
    }

    fn handle_register_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let register_requests = chunk_manager.register_requests.clone();
        let processed_register_requests = Vec::new();
        let failed_register_requests = Vec::new();

        for id in register_requests {
            let chunk = chunk_manager.get_chunk(id);
            // register the chunk
        }

        for failed_register_request in failed_register_requests {
            println!("Failed to register chunk: {}", failed_register_request);
        }

        chunk_manager.register_requests.clear();
    }

    fn handle_unregister_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let unregister_requests = chunk_manager.unregister_requests.clone();
        let processed_unregister_requests = Vec::new();
        let failed_unregister_requests = Vec::new();

        for id in unregister_requests {
            let chunk = chunk_manager.get_chunk(id);
            // unregister the chunk
        }

        for failed_unregister_request in failed_unregister_requests {
            println!("Failed to unregister chunk: {}", failed_unregister_request);
        }

        chunk_manager.unregister_requests.clear();
    }

    fn handle_load_metadata_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let load_metadata_requests = chunk_manager.load_metadata_requests.clone();
        let processed_load_metadata_requests = Vec::new();
        let failed_load_metadata_requests = Vec::new();

        for id in load_metadata_requests {
            let chunk = chunk_manager.get_chunk(id);
            // load the metadata
        }

        for failed_load_metadata_request in failed_load_metadata_requests {
            println!("Failed to load chunk metadata: {}", failed_load_metadata_request);
        }

        chunk_manager.load_metadata_requests.clear();
    }

    fn handle_unload_metadata_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let unload_metadata_requests = chunk_manager.unload_metadata_requests.clone();
        let processed_unload_metadata_requests = Vec::new();
        let failed_unload_metadata_requests = Vec::new();

        for id in unload_metadata_requests {
            let chunk = chunk_manager.get_chunk(id);
            // unload the metadata
        }

        for failed_unload_metadata_request in failed_unload_metadata_requests {
            println!("Failed to unload chunk metadata: {}", failed_unload_metadata_request);
        }

        chunk_manager.unload_metadata_requests.clear();
    }

    fn handle_load_data_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let load_data_requests = chunk_manager.load_data_requests.clone();
        let processed_load_data_requests = Vec::new();
        let failed_load_data_requests = Vec::new();

        for id in load_data_requests {
            let chunk = chunk_manager.get_chunk(id);
            // load the data
        }

        for failed_load_data_request in failed_load_data_requests {
            println!("Failed to load chunk data: {}", failed_load_data_request);
        }

        chunk_manager.load_data_requests.clear();
    }

    fn handle_unload_data_requests(mut chunk_manager: ResMut<ChunkManager>) {
        let unload_data_requests = chunk_manager.unload_data_requests.clone();
        let processed_unload_data_requests = Vec::new();
        let failed_unload_data_requests = Vec::new();

        for id in unload_data_requests {
            let chunk = chunk_manager.get_chunk(id);
            // unload the data
        }

        for failed_unload_data_request in failed_unload_data_requests {
            println!("Failed to unload chunk data: {}", failed_unload_data_request);
        }

        chunk_manager.unload_data_requests.clear();
    }

    fn handle_spawn_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let spawn_requests = chunk_manager.spawn_requests.clone();
        let processed_spawn_requests = Vec::new();
        let failed_spawn_requests = Vec::new();

        for id in spawn_requests {
            let chunk = chunk_manager.get_chunk(id);
            // spawn the chunk
        }

        for failed_spawn_request in failed_spawn_requests {
            println!("Failed to spawn chunk: {}", failed_spawn_request);
        }

        chunk_manager.spawn_requests.clear();
    }

    fn handle_despawn_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let despawn_requests = chunk_manager.despawn_requests.clone();
        let processed_despawn_requests = Vec::new();
        let failed_despawn_requests = Vec::new();

        for id in despawn_requests {
            let chunk = chunk_manager.get_chunk(id);
            // despawn the chunk
        }

        for failed_despawn_request in failed_despawn_requests {
            println!("Failed to despawn chunk: {}", failed_despawn_request);
        }

        chunk_manager.despawn_requests.clear();
    }
}

// Module Functions
