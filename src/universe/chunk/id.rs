// Modules

// Local imports

// Internal imports
use crate::math::*;
use crate::universe::chunk::pos::*;
use crate::universe::entity::id::*;

// External imports
use num_bigint::BigUint;

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Eq, Hash, Clone, Debug)]
pub struct ChunkID {
    global_id_base10: BigUint,
    global_id_base10x10: Vec<(u8, u8)>,
    global_id_base57: String,
    scale_index: u8,
}

// Implementations
impl From<EntityID> for ChunkID {
    fn from(entity_id: EntityID) -> Self {
        entity_id.get_parent_chunk_id().clone()
    }
}

impl TryFrom<(u8, u8)> for ChunkID {
    type Error = String;

    fn try_from(global_id_base10x10: (u8, u8)) -> Result<Self, Self::Error> {
        let global_id_base10 = BASE10X10_CONVERTER
            .convert_from_base10x10(vec![global_id_base10x10.clone()])
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base57 = BASE57_CONVERTER
            .convert_to_base57(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base57 ID failed: {}", e))?;

        let mut chunk_id = ChunkID {
            global_id_base10,
            global_id_base10x10: vec![global_id_base10x10],
            global_id_base57,
            scale_index: 0,
        };

        Ok(chunk_id)
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

        let scale_index = global_id_base10x10.len() as u8 - 1;

        let chunk_id = ChunkID {
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
            scale_index,
        };

        Ok(chunk_id)
    }
}

impl TryFrom<Vec<(u8, u8)>> for ChunkID {
    type Error = String;

    fn try_from(global_id_base10x10: Vec<(u8, u8)>) -> Result<Self, Self::Error> {
        if global_id_base10x10.is_empty() {
            return Err("Cannot convert empty vector to chunk ID.".to_string());
        }

        let global_id_base10 = BASE10X10_CONVERTER
            .convert_from_base10x10(global_id_base10x10.clone())
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base57 = BASE57_CONVERTER
            .convert_to_base57(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base57 ID failed: {}", e))?;

        let scale_index = global_id_base10x10.len() as u8 - 1;

        let chunk_id = ChunkID {
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
            scale_index,
        };

        Ok(chunk_id)
    }
}

impl TryFrom<&str> for ChunkID {
    type Error = String;

    fn try_from(global_id_base57: &str) -> Result<Self, Self::Error> {
        if global_id_base57.is_empty() {
            return Err("Cannot convert empty string to chunk ID.".to_string());
        }

        let global_id_base10 = BASE57_CONVERTER
            .convert_from_base57(global_id_base57.clone())
            .map_err(|e| format!("Computing the Base10 ID failed: {}", e))?;
        let global_id_base10x10 = BASE10X10_CONVERTER
            .convert_to_base10x10(global_id_base10.clone())
            .map_err(|e| format!("Computing the Base10x10 ID failed: {}", e))?;

        let scale_index = global_id_base10x10.len() as u8 - 1;

        let chunk_id = ChunkID {
            global_id_base10,
            global_id_base10x10,
            global_id_base57: global_id_base57.to_string(),
            scale_index,
        };

        Ok(chunk_id)
    }
}

impl PartialEq for ChunkID {
    fn eq(&self, other: &Self) -> bool {
        self.global_id_base10x10 == other.global_id_base10x10
    }
}

impl Default for ChunkID {
    fn default() -> Self {
        Self {
            global_id_base10: BigUint::from(0u8),
            global_id_base10x10: vec![(0u8, 0u8)],
            global_id_base57: "0".to_string(),
            scale_index: 0,
        }
    }
}

impl ChunkID {
    pub fn get_global_id_base10(&self) -> &BigUint {
        return &self.global_id_base10;
    }

    pub fn get_global_id_base10x10(&self) -> &Vec<(u8, u8)> {
        return &self.global_id_base10x10;
    }

    pub fn get_global_id_base57(&self) -> &String {
        return &self.global_id_base57;
    }

    pub fn get_scale_index(&self) -> &u8 {
        return &self.scale_index;
    }

    pub fn compute_parent_id(&self) -> Result<ChunkID, String> {
        if self.scale_index == 0 {
            return Err(
                "Cannot compute parent ID: Chunk ID is already a root chunk ID.".to_string(),
            );
        }

        let mut id_base10x10 = self.global_id_base10x10.clone();
        id_base10x10.pop();

        return ChunkID::try_from(id_base10x10);
    }

    pub fn compute_local_pos(&self) -> Result<LocalChunkPos, String> {
        let local_pos_base10x10 = match self.global_id_base10x10.last() {
            Some(local_pos_base10x10) => local_pos_base10x10.clone(),
            None => {
                return Err(
                    "Cannot compute local position from chunk ID: Chunk ID is invalid.".to_string(),
                )
            }
        };

        return Ok(LocalChunkPos::from(local_pos_base10x10));
    }

    pub fn compute_pos(&self) -> Result<ChunkPos, String> {
        let mut id_base10x10 = self.global_id_base10x10.clone();

        if id_base10x10.is_empty() {
            return Err("Cannot compute position from chunk ID: Chunk ID is invalid.".to_string());
        }

        let first_local_pos_base10x10 = id_base10x10.remove(0);
        let mut chunk_pos = ChunkPos::new(None, first_local_pos_base10x10.into());

        for local_pos_base10x10 in id_base10x10 {
            chunk_pos = ChunkPos::new(Some(Box::new(chunk_pos)), local_pos_base10x10.into());
        }

        Ok(chunk_pos)
    }
}

// Module Functions
