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
    loaded_chunks: HashMap<ChunkPos, bevy::prelude::Entity>,
    load_requests: Vec<ChunkPos>,
    unload_requests: Vec<ChunkPos>,
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

    pub fn load_metadata(&mut self, metadata: ChunkMetadata) -> Result<(), String> {
        match self {
            Chunk::Registered { .. } => {
                *self = Chunk::MetadataLoaded {
                    id: self.get_id().clone(),
                    metadata: Arc::new(Mutex::new(metadata)),
                };
                Ok(())
            },
            Chunk::MetadataLoaded { .. } => {
                Err("Cannot load metadata: Metadata is already loaded.".to_string())
            }
            Chunk::DataLoaded { .. } => {
                Err("Cannot load metadata: Both metadata and data are already loaded.".to_string())
            }
        }
    }

    pub fn load_data(&mut self, data: ChunkData) -> Result<(), String> {
        match self {
            Chunk::Registered { .. } => {
                Err("Cannot load data: Metadata must be loaded first.".to_string())
            }
            Chunk::MetadataLoaded { .. } => {
                *self = Chunk::DataLoaded {
                    id: self.get_id().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                    data: Arc::new(Mutex::new(data)),
                };
                Ok(())
            },
            Chunk::DataLoaded { .. } => {
                Err("Cannot load data: Data is already loaded.".to_string())
            }
        }
    }

    pub fn unload_metadata(&mut self) -> Result<(), String> {
        match self {
            Chunk::Registered { .. } => {
                Err("Cannot unload metadata: No metadata is loaded.".to_string())
            }
            Chunk::MetadataLoaded { .. } => {
                *self = Chunk::Registered {
                    id: self.get_id().clone(),
                };
                Ok(())
            }
            Chunk::DataLoaded { .. } => {
                Err("Cannot unload metadata: Data must be unloaded first.".to_string())
            }
        }
    }

    pub fn unload_data(&mut self) -> Result<(), String> {
        match self {
            Chunk::Registered { .. } => {
                Err("Cannot unload data: Neither metadata nor data are loaded.".to_string())
            }
            Chunk::MetadataLoaded { .. } => {
                Err("Cannot unload data: No data is loaded.".to_string())
            }
            Chunk::DataLoaded { .. } => {
                *self = Chunk::MetadataLoaded {
                    id: self.get_id().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                };
                Ok(())
            }
        }
    }

    pub fn get_id(&self) -> Arc<RwLock<ChunkID>> {
        match self {
            Chunk::Registered { id } => id.clone(),
            Chunk::MetadataLoaded { id, .. } => id.clone(),
            Chunk::DataLoaded { id, .. } => id.clone(),
        }
    }

    pub fn get_metadata(&self) -> Result<Arc<Mutex<ChunkMetadata>>, String> {
        match self {
            Chunk::Registered { .. } => Err("No metadata is loaded.".to_string()),
            Chunk::MetadataLoaded { metadata, .. } => Ok(metadata.clone()),
            Chunk::DataLoaded { metadata, .. } => Ok(metadata.clone()),
        }
    }

    pub fn get_data(&self) -> Result<Arc<Mutex<ChunkData>>, String> {
        match self {
            Chunk::Registered { .. } => Err("No data is loaded.".to_string()),
            Chunk::MetadataLoaded { .. } => Err("No data is loaded.".to_string()),
            Chunk::DataLoaded { data, .. } => Ok(data.clone()),
        }
    }

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
            loaded_chunks: HashMap::new(),
            load_requests: Vec::new(),
            unload_requests: Vec::new(),
        };

        commands.insert_resource(chunk_manager);
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<Self>();
    }

    pub fn request_load(&mut self, chunk_pos: ChunkPos) {
        self.load_requests.push(chunk_pos);
    }

    pub fn request_unload(&mut self, chunk_pos: ChunkPos) {
        self.unload_requests.push(chunk_pos);
    }

    fn handle_load_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let load_requests = chunk_manager.load_requests.clone();
        for chunk_pos in load_requests {
            match chunk_manager.loaded_chunks.get(&chunk_pos) {
                Some(_) => panic!("Chunk already loaded"),
                None => {
                    let chunk_entity = commands
                        .spawn(Chunk {
                            local_chunk_pos: chunk_pos,
                        })
                        .id();
                    chunk_manager.loaded_chunks.insert(chunk_pos, chunk_entity);
                }
            }
        }
        chunk_manager.load_requests.clear();
    }

    fn handle_unload_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let unload_requests = chunk_manager.unload_requests.clone();
        for chunk_pos in unload_requests {
            match chunk_manager.loaded_chunks.get(&chunk_pos) {
                Some(chunk_entity) => {
                    commands.entity(*chunk_entity).despawn();
                    chunk_manager.loaded_chunks.remove(&chunk_pos);
                }
                None => panic!("Chunk not loaded"),
            }
        }
        chunk_manager.unload_requests.clear();
    }
}

// Module Functions
