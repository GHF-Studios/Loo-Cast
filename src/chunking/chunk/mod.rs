// Modules


// Local imports


// Internal imports
use crate::math::*;
use super::cluster::*;
use super::entity::*;
use crate::chunking::identification::LocalID::*;

// External imports
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

// Static variables


// Constant variables


// Types


// Enums
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
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkID {
    parent_cluster_id: Option<ClusterID>
}

pub struct ChunkMetadata {
    current_local_entity_id: u64,
    parent_cluster: Arc<Mutex<Cluster>>,
    registered_entities: Hashmap<u64, Arc<Mutex<Entity>>>,
}

pub struct ChunkData {
    placeholder_data: Option<i32>,
}

// Implementations
impl From<EntityID> for ChunkID {
    fn from(entity_id: EntityID) -> Self {
        entity_id.get_chunk_id()
    }
}

impl PartialEq for ChunkID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10x10 == other.global_id_base10x10
    }
}

impl ChunkID {
    pub fn new(cluster_id: Option<ClusterID>, local_id: u8) -> Result<ChunkID, String> {
        if local_id > 99u8 {
            return Err("Invalid local chunk id");
        }

        let y = local_id % 10u8;
        let x = local_id / 10u8;

        Ok(ChunkID {
            cluster_id,
            local_id,
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
        })
    }

    pub fn get_cluster_id(&self) -> &ClusterID {
        return self.cluster_id.as_ref().unwrap();
    }

    pub fn get_local_id(&self) -> u8 {
        return self.local_id;
    }

    pub fn get_scale_level(&self) -> u8 {
        return self.scale_level;
    }
}

impl ChunkMetadata {
    pub fn new(current_local_entity_id: u64, parent_cluster: Arc<Mutex<Cluster>>) -> Self {
        ChunkMetadata {
            current_local_entity_id,
            parent_cluster,
            registered_entities: Vec::new(),
        }
    }

    pub fn generate_entity_id(&mut self, chunk_id: ChunkID) -> EntityID {
        match EntityID::new(chunk_id, self.current_local_entity_id) {
            Ok(entity_id) => {
                self.current_local_entity_id += 1;
                return entity_id;
            },
            Err(e) => panic!("Generating a local entity id failed: {}", e),
        }
    }

    pub fn register_entity(&mut self, entity_id: EntityID) -> Arc<Mutex<Entity>> {
        if self.registered_entities.contains_key(&entity_id) {
            panic!("Entity already registered.");
        }

        let entity = Arc::new(Mutex::new(Entity::Registered { id: entity_id.clone(), }));
        self.registered_entities.insert(entity_id, entity.clone());
        entity
    }

    pub fn get_registered_entity(&mut self, entity_id: EntityID) -> Option<Arc<Mutex<Entity>>> {
        self.registered_entities.get(&entity_id).map(|entity| entity.clone())
    }

    pub fn is_entity_registered(&mut self, entity_id: EntityID) -> bool {
        self.registered_entities.contains_key(&entity_id)
    }
}

// Module Functions
