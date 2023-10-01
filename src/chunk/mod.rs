// Internal imports
use crate::math::*;

// External imports
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::sync::{Arc, Mutex, RwLock};

// Enums
pub enum ChunkCluster {
    Registered {
        registration: Arc<RwLock<ChunkClusterRegistration>>,
    },
    MetadataLoaded {
        registration: Arc<RwLock<ChunkClusterRegistration>>,
        metadata: Arc<Mutex<ChunkClusterMetadata>>,
    },
    DataLoaded {
        registration: Arc<RwLock<ChunkClusterRegistration>>,
        metadata: Arc<Mutex<ChunkClusterMetadata>>,
        data: Arc<Mutex<ChunkClusterData>>,
    },
}

pub enum ChunkClusterLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

// Structs
#[derive(Clone, Debug)]
pub struct ChunkClusterID {
    global_id_base10: BigUint,
    global_id_base57: String,
}

pub struct ChunkClusterRegistration {
    id: ChunkClusterID,
}

pub struct ChunkClusterMetadata {
    placeholder_metadata: Option<i32>,
}

pub struct ChunkClusterData {
    placeholder_data: Option<i32>,
}

pub struct ChunkClusterManager {
    registered_clusters: Arc<Mutex<Vec<ChunkCluster>>>,
}

#[derive(Clone, Debug)]
pub struct ChunkID {
    scale_level: u8,
    local_id: u8,
    cluster_id: Option<ChunkClusterID>,
    global_id_base10: BigUint,
    global_id_base10x10: Vec<(u8, u8)>,
    global_id_base57: String,
}

// Implementations
impl ChunkClusterID {
    pub fn from_chunk_id(chunk_id: ChunkID) -> ChunkClusterID {
        let base10_id = chunk_id.get_global_id_base10() / BigUint::from(100u32);
        let base57_id = BASE57_CONVERTER
            .convert_to_base57(base10_id.clone())
            .unwrap();

        ChunkClusterID {
            global_id_base10: base10_id,
            global_id_base57: base57_id,
        }
    }

    pub fn get_global_id_base10(&self) -> &BigUint {
        return &self.global_id_base10;
    }

    pub fn get_global_id_base57(&self) -> &String {
        return &self.global_id_base57;
    }
}

impl PartialEq for ChunkClusterID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10 == other.global_id_base10
    }
}

impl ChunkClusterRegistration {
    fn new(id: ChunkClusterID) -> ChunkClusterRegistration {
        ChunkClusterRegistration { id: id }
    }

    fn get_id(&self) -> &ChunkClusterID {
        return &self.id;
    }
}

impl ChunkClusterMetadata {
    fn new() -> ChunkClusterMetadata {
        ChunkClusterMetadata {
            placeholder_metadata: None,
        }
    }

    fn get_placeholder_metadata(&self) -> Option<i32> {
        return self.placeholder_metadata;
    }

    fn set_placeholder_metadata(&mut self, placeholder_metadata: Option<i32>) {
        self.placeholder_metadata = placeholder_metadata;
    }
}

impl ChunkClusterData {
    fn new() -> ChunkClusterData {
        ChunkClusterData {
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

impl ChunkCluster {
    fn new(id: ChunkClusterID) -> Self {
        ChunkCluster::Registered {
            registration: Arc::new(RwLock::new(ChunkClusterRegistration::new(id))),
        }
    }

    fn load_metadata(&mut self, metadata: ChunkClusterMetadata) -> Result<(), String> {
        match self {
            ChunkCluster::Registered { .. } => {
                *self = ChunkCluster::MetadataLoaded {
                    registration: self.get_registration().clone(),
                    metadata: Arc::new(Mutex::new(metadata)),
                };
                Ok(())
            },
            ChunkCluster::MetadataLoaded { .. } => {
                Err("Cannot load metadata: Metadata is already loaded.".to_string())
            }
            ChunkCluster::DataLoaded { .. } => {
                Err("Cannot load metadata: Both metadata and data are already loaded.".to_string())
            }
        }
    }

    fn load_data(&mut self, data: ChunkClusterData) -> Result<(), String> {
        match self {
            ChunkCluster::Registered { .. } => {
                Err("Cannot load data: Metadata must be loaded first.".to_string())
            }
            ChunkCluster::MetadataLoaded { .. } => {
                *self = ChunkCluster::DataLoaded {
                    registration: self.get_registration().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                    data: Arc::new(Mutex::new(data)),
                };
                Ok(())
            },
            ChunkCluster::DataLoaded { .. } => {
                Err("Cannot load data: Data is already loaded.".to_string())
            }
        }
    }

    fn unload_metadata(&mut self) -> Result<(), String> {
        match self {
            ChunkCluster::Registered { .. } => {
                Err("Cannot unload metadata: No metadata is loaded.".to_string())
            }
            ChunkCluster::MetadataLoaded { .. } => {
                *self = ChunkCluster::Registered {
                    registration: self.get_registration().clone(),
                };
                Ok(())
            }
            ChunkCluster::DataLoaded { .. } => {
                Err("Cannot unload metadata: Data must be unloaded first.".to_string())
            }
        }
    }

    fn unload_data(&mut self) -> Result<(), String> {
        match self {
            ChunkCluster::Registered { .. } => {
                Err("Cannot unload data: Neither metadata nor data are loaded.".to_string())
            }
            ChunkCluster::MetadataLoaded { .. } => {
                Err("Cannot unload data: No data is loaded.".to_string())
            }
            ChunkCluster::DataLoaded { .. } => {
                *self = ChunkCluster::MetadataLoaded {
                    registration: self.get_registration().clone(),
                    metadata: self.get_metadata().unwrap().clone(),
                };
                Ok(())
            }
        }
    }

    fn get_registration(&self) -> &Arc<RwLock<ChunkClusterRegistration>> {
        match self {
            ChunkCluster::Registered { registration } => registration,
            ChunkCluster::MetadataLoaded { registration, .. } => registration,
            ChunkCluster::DataLoaded { registration, .. } => registration,
        }
    }

    fn get_metadata(&self) -> Result<&Arc<Mutex<ChunkClusterMetadata>>, String> {
        match self {
            ChunkCluster::Registered { .. } => Err("No metadata is loaded.".to_string()),
            ChunkCluster::MetadataLoaded { metadata, .. } => Ok(metadata),
            ChunkCluster::DataLoaded { metadata, .. } => Ok(metadata),
        }
    }

    fn get_data(&self) -> Result<&Arc<Mutex<ChunkClusterData>>, String> {
        match self {
            ChunkCluster::Registered { .. } => Err("No data is loaded.".to_string()),
            ChunkCluster::MetadataLoaded { .. } => Err("No data is loaded.".to_string()),
            ChunkCluster::DataLoaded { data, .. } => Ok(data),
        }
    }

    fn get_load_state(&self) -> ChunkClusterLoadState {
        match self {
            ChunkCluster::Registered { .. } => ChunkClusterLoadState::Registered,
            ChunkCluster::MetadataLoaded { .. } => ChunkClusterLoadState::MetadataLoaded,
            ChunkCluster::DataLoaded { .. } => ChunkClusterLoadState::DataLoaded,
        }
    }
}

impl ChunkClusterManager {
}

impl ChunkID {
    pub fn from_base10_global(global_id_base10: BigUint) -> Result<ChunkID, String> {
        let global_id_base10x10 = BASE10X10_CONVERTER
            .convert_to_base10x10(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base10x10 ID failed: {}", e))?;
        let global_id_base57 = BASE57_CONVERTER
            .convert_to_base57(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base57 ID failed: {}", e))?;

        let mut chunk_id = ChunkID {
            scale_level: global_id_base10x10.len() as u8,
            local_id: (global_id_base10.clone() % BigUint::from(100u32)).to_u8().unwrap(),
            cluster_id: None,
            global_id_base10: global_id_base10,
            global_id_base10x10,
            global_id_base57,
        };
        chunk_id.cluster_id = Some(ChunkClusterID::from_chunk_id(chunk_id.clone()));

        Ok(chunk_id)
    }

    pub fn from_base10x10_global(global_id_base10x10: Vec<(u8, u8)>) -> Result<ChunkID, String> {
        let global_id_base10 = BASE10X10_CONVERTER
            .convert_from_base10x10(global_id_base10x10.clone())
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base57 = BASE57_CONVERTER
            .convert_to_base57(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base57 ID failed: {}", e))?;

        let mut chunk_id = ChunkID {
            scale_level: global_id_base10x10.len() as u8,
            local_id: (global_id_base10.clone() % BigUint::from(100u32)).to_u8().unwrap(),
            cluster_id: None,
            global_id_base10,
            global_id_base10x10: global_id_base10x10,
            global_id_base57,
        };
        chunk_id.cluster_id = Some(ChunkClusterID::from_chunk_id(chunk_id.clone()));

        Ok(chunk_id)
    }

    pub fn from_base57_global(global_id_base57: &str) -> Result<ChunkID, String> {
        let global_id_base10 = BASE57_CONVERTER
            .convert_from_base57(global_id_base57.clone())
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base10x10 = BASE10X10_CONVERTER
            .convert_to_base10x10(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base10x10 ID failed: {}", e))?;

        let mut chunk_id = ChunkID {
            scale_level: global_id_base10x10.len() as u8,
            local_id: (global_id_base10.clone() % BigUint::from(100u32)).to_u8().unwrap(),
            cluster_id: None,
            global_id_base10,
            global_id_base10x10,
            global_id_base57: global_id_base57.to_string(),
        };
        chunk_id.cluster_id = Some(ChunkClusterID::from_chunk_id(chunk_id.clone()));

        Ok(chunk_id)
    }

    pub fn get_scale_level(&self) -> u8 {
        return self.scale_level;
    }

    pub fn get_local_id(&self) -> u8 {
        return self.local_id;
    }

    pub fn get_cluster_id(&self) -> &ChunkClusterID {
        return self.cluster_id.as_ref().unwrap();
    }

    pub fn get_global_id_base10(&self) -> &BigUint {
        return &self.global_id_base10;
    }

    pub fn get_global_id_base10x10(&self) -> &Vec<(u8, u8)> {
        return &self.global_id_base10x10;
    }

    pub fn get_global_id_base57(&self) -> &String {
        return &self.global_id_base57;
    }
}

impl PartialEq for ChunkID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10x10 == other.global_id_base10x10
    }
}
