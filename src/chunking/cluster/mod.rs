// Modules


// Local imports


// Internal imports
use crate::math::*;
use super::chunk::*;

// External imports
use num_bigint::BigUint;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

// Static variables


// Constant variables


// Types


// Enums
pub enum Cluster {
    Registered {
        id: Arc<RwLock<ClusterID>>,
    },
    MetadataLoaded {
        id: Arc<RwLock<ClusterID>>,
        metadata: Arc<Mutex<ClusterMetadata>>,
    },
    DataLoaded {
        id: Arc<RwLock<ClusterID>>,
        metadata: Arc<Mutex<ClusterMetadata>>,
        data: Arc<Mutex<ClusterData>>,
    },
}

pub enum ClusterLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

// Structs
#[derive(Clone, Debug, PartialEq)]
pub struct ClusterID {
    global_id_base10: BigUint,
    global_id_base10x10: Vec<(u8, u8)>,
    global_id_base57: String,
    scale_level: u8,
}

pub struct ClusterMetadata {
    parent_cluster: Option<Arc<Mutex<Cluster>>>,
    child_clusters: Option<HashMap<u8, Cluster>>,
    child_chunk: Arc<Mutex<Chunk>>,
}

pub struct ClusterData {
    placeholder_data: Option<i32>,
}

// Implementations
impl From<EntityID> for ClusterID {
    fn from(entity_id: EntityID) -> Self {
        entity_id.get_chunk_id().get_cluster_id()
    }
}

impl From<ChunkID> for ClusterID {
    fn from(chunk_id: ChunkID) -> Self {
        chunk_id.get_cluster_id()
    }
}

impl TryFrom<BigUint> for ClusterID {
    type Error = String;

    fn try_from(global_id_base10: BigUint) -> Result<Self, Self::Error> {
        let global_id_base10x10 = BASE10X10_CONVERTER
            .convert_to_base10x10(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base10x10 ID failed: {}", e))?;
        let global_id_base57 = BASE57_CONVERTER
            .convert_to_base57(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base57 ID failed: {}", e))?;

        let mut chunk_id = ClusterID {
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
            scale_level: global_id_base10x10.len() as u8,
        };

        Ok(chunk_id)
    }
}

impl TryFrom<Vec<(u8, u8)>> for ClusterID {
    type Error = String;

    fn try_from(global_id_base10x10: Vec<(u8, u8)>) -> Result<Self, Self::Error> {
        let global_id_base10 = BASE10X10_CONVERTER
            .convert_from_base10x10(global_id_base10x10.clone())
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base57 = BASE57_CONVERTER
            .convert_to_base57(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base57 ID failed: {}", e))?;

        let mut chunk_id = ChunkID {
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
            scale_level: global_id_base10x10.len() as u8,
        };

        Ok(chunk_id)
    }
}

impl TryFrom<&str> for ClusterID {
    type Error = String;

    fn try_from(global_id_base57: &str) -> Result<Self, Self::Error> {
        let global_id_base10 = BASE57_CONVERTER
            .convert_from_base57(global_id_base57.clone())
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base10x10 = BASE10X10_CONVERTER
            .convert_to_base10x10(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base10x10 ID failed: {}", e))?;

        let mut chunk_id = ChunkID {
            global_id_base10,
            global_id_base10x10,
            global_id_base57: global_id_base57.to_string(),
            scale_level: global_id_base10x10.len() as u8,
        };

        Ok(chunk_id)
    }
}

impl PartialEq for ClusterID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10 == other.global_id_base10
    }
}

impl ClusterID {
    pub fn get_global_id_base10(&self) -> &BigUint {
        return &self.global_id_base10;
    }

    pub fn get_global_id_base10x10(&self) -> &Vec<(u8, u8)> {
        return &self.global_id_base10x10;
    }

    pub fn get_global_id_base57(&self) -> &String {
        return &self.global_id_base57;
    }

    pub fn get_scale_level(&self) -> u8 {
        return self.scale_level;
    }
}

impl ClusterMetadata {
    fn new(parent_cluster: Option<Arc<Mutex<Cluster>>>, child_chunk: Arc<Mutex<Chunk>>) -> Result<ClusterMetadata, String> {
        if let Some(parent_cluster) = parent_cluster {
            let parent_scale_index = parent_cluster.lock().unwrap().get_global_id_base10x10().len() - 1;
            if parent_scale_index < 63 {
                return Ok(ClusterMetadata {
                    parent_cluster,
                    child_clusters: Some(HashMap::new()), 
                    child_chunk
                });
            } else if parent_scale_index == 63 {
                return Ok(ClusterMetadata {
                    parent_cluster,
                    child_clusters: None, 
                    child_chunk
                });
            } else if parent_scale_index > 63 {
                panic!("Cannot create a cluster with a scale index higher than 63.");
            }
        } else {
            return Ok(ClusterMetadata {
                None,
                child_clusters: HashMap::new(), 
                child_chunk
            });
        }
    }

    pub fn register_child_cluster(&mut self, chunk_id: ChunkID) -> Arc<Mutex<Chunk>> {
        if self.registered_chunks.contains_key(&chunk_id) {
            panic!("Chunk already registered.");
        }

        let chunk = Arc::new(Mutex::new(Chunk::Registered { id: chunk_id.clone() }));
        self.registered_chunks.insert(chunk_id, chunk.clone());
        chunk
    }

    pub fn get_registered_chunk(&mut self, chunk_id: ChunkID) -> Option<Arc<Mutex<Chunk>>> {
        self.registered_chunks.get(&chunk_id).map(|chunk| chunk.clone())
    }

    pub fn is_chunk_registered(&mut self, chunk_id: ChunkID) -> bool {
        self.registered_chunks.contains_key(&chunk_id)
    }
}

impl ClusterData {
    fn new() -> ClusterData {
        ClusterData {
            placeholder_data: None,
        }
    }

    fn get_placeholder_data(&self) -> Option<i32> {
        return self.placeholder_data;
    }

    fn set_placeholder_data(&mut self, placeholder_data: Option<i32>) {
        self.placeholder_data = placeholder_data;
    }
}

impl Cluster {
    fn new(id: ClusterID) -> Self {
        Cluster::Registered {
            id: Arc::new(RwLock::new(ClusterID::new(id))),
        }
    }

    fn load_metadata(&mut self, metadata: ClusterMetadata) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                *self = Cluster::MetadataLoaded {
                    id: self.get_id().clone(),
                    metadata: Arc::new(Mutex::new(metadata)),
                };
                Ok(())
            },
            Cluster::MetadataLoaded { .. } => {
                Err("Cannot load metadata: Metadata is already loaded.".to_string())
            }
            Cluster::DataLoaded { .. } => {
                Err("Cannot load metadata: Both metadata and data are already loaded.".to_string())
            }
        }
    }

    fn load_data(&mut self, data: ClusterData) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                Err("Cannot load data: Metadata must be loaded first.".to_string())
            }
            Cluster::MetadataLoaded { .. } => {
                *self = Cluster::DataLoaded {
                    id: self.get_id().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                    data: Arc::new(Mutex::new(data)),
                };
                Ok(())
            },
            Cluster::DataLoaded { .. } => {
                Err("Cannot load data: Data is already loaded.".to_string())
            }
        }
    }

    fn unload_metadata(&mut self) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                Err("Cannot unload metadata: No metadata is loaded.".to_string())
            }
            Cluster::MetadataLoaded { .. } => {
                *self = Cluster::Registered {
                    id: self.get_id().clone(),
                };
                Ok(())
            }
            Cluster::DataLoaded { .. } => {
                Err("Cannot unload metadata: Data must be unloaded first.".to_string())
            }
        }
    }

    fn unload_data(&mut self) -> Result<(), String> {
        match self {
            Cluster::Registered { .. } => {
                Err("Cannot unload data: Neither metadata nor data are loaded.".to_string())
            }
            Cluster::MetadataLoaded { .. } => {
                Err("Cannot unload data: No data is loaded.".to_string())
            }
            Cluster::DataLoaded { .. } => {
                *self = Cluster::MetadataLoaded {
                    id: self.get_id().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                };
                Ok(())
            }
        }
    }

    fn get_id(&self) -> &Arc<RwLock<ClusterID>> {
        match self {
            Cluster::Registered { id } => id,
            Cluster::MetadataLoaded { id, .. } => id,
            Cluster::DataLoaded { id, .. } => id,
        }
    }

    fn get_metadata(&self) -> Result<&Arc<Mutex<ClusterMetadata>>, String> {
        match self {
            Cluster::Registered { .. } => Err("No metadata is loaded.".to_string()),
            Cluster::MetadataLoaded { metadata, .. } => Ok(metadata),
            Cluster::DataLoaded { metadata, .. } => Ok(metadata),
        }
    }

    fn get_data(&self) -> Result<&Arc<Mutex<ClusterData>>, String> {
        match self {
            Cluster::Registered { .. } => Err("No data is loaded.".to_string()),
            Cluster::MetadataLoaded { .. } => Err("No data is loaded.".to_string()),
            Cluster::DataLoaded { data, .. } => Ok(data),
        }
    }

    fn get_load_state(&self) -> ClusterLoadState {
        match self {
            Cluster::Registered { .. } => ClusterLoadState::Registered,
            Cluster::MetadataLoaded { .. } => ClusterLoadState::MetadataLoaded,
            Cluster::DataLoaded { .. } => ClusterLoadState::DataLoaded,
        }
    }
}

// Module Functions
