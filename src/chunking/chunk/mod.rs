// Modules


// Local imports


// Internal imports
use crate::math::*;
use super::cluster::*;
use super::entity::*;

// External imports
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::sync::{Arc, Mutex, RwLock};

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
    scale_level: u8,
    local_id: u8,
    cluster_id: Option<ClusterID>,
    global_id_base10: BigUint,
    global_id_base10x10: Vec<(u8, u8)>,
    global_id_base57: String,
}

pub struct ChunkMetadata {
    placeholder_metadata: Option<i32>,
}

pub struct ChunkData {
    placeholder_data: Option<i32>,
}

pub struct ChunkManager {
    registered_chunks: Arc<Mutex<Vec<Chunk>>>,
}


// Implementations
impl From<EntityID> for ChunkID {
    fn from(entity_id: EntityID) -> Self {
        entity_id.get_chunk_id()
    }
}

impl TryFrom<BigUint> for ChunkID {
    type Error = String;

    fn try_from(global_id_base10: BigUint) -> Result<Self, Self::Error> {
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
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
        };
        chunk_id.cluster_id = Some(ClusterID::from_chunk_id(chunk_id.clone()));

        Ok(chunk_id)
    }
}

impl TryFrom<Vec<(u8, u8)>> for ChunkID {
    type Error = String;

    fn try_from(global_id_base10x10: Vec<(u8, u8)>) -> Result<Self, Self::Error> {
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
            global_id_base10x10,
            global_id_base57,
        };
        chunk_id.cluster_id = Some(ClusterID::from_chunk_id(chunk_id.clone()));

        Ok(chunk_id)
    }
}

impl TryFrom<&str> for ChunkID {
    type Error = String;

    fn try_from(global_id_base57: &str) -> Result<Self, Self::Error> {
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
        chunk_id.cluster_id = Some(ClusterID::from_chunk_id(chunk_id.clone()));

        Ok(chunk_id)
    }
}

impl PartialEq for ChunkID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10x10 == other.global_id_base10x10
    }
}

impl ChunkID {
    pub fn get_scale_level(&self) -> u8 {
        return self.scale_level;
    }

    pub fn get_local_id(&self) -> u8 {
        return self.local_id;
    }

    pub fn get_cluster_id(&self) -> &ClusterID {
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

// Module Functions
