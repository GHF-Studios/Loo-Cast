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
#[derive(Eq, Hash, Clone, Copy, Debug)]
pub struct LocalChunkID {
    id: u8,
}

#[derive(Eq, Hash, Clone, Debug)]
pub struct ChunkID {
    scale_index: u8,
    global_id_base10: BigUint,
    global_id_base10x10: Vec<(u8, u8)>,
    global_id_base57: String,
}

// Implementations
impl PartialEq for LocalChunkID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for LocalChunkID {
    fn default() -> Self {
        LocalChunkID { id: 0 }
    }
}

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

        let chunk_id = ChunkID {
            scale_index: 0,
            global_id_base10,
            global_id_base10x10: vec![global_id_base10x10],
            global_id_base57,
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
            scale_index,
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
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
            scale_index,
            global_id_base10,
            global_id_base10x10,
            global_id_base57,
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
            scale_index,
            global_id_base10,
            global_id_base10x10,
            global_id_base57: global_id_base57.to_string(),
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
            scale_index: 0,
            global_id_base10: BigUint::from(0u8),
            global_id_base10x10: vec![(0u8, 0u8)],
            global_id_base57: "0".to_string(),
        }
    }
}

impl ChunkID {
    pub fn get_scale_index(&self) -> &u8 {
        return &self.scale_index;
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

    pub fn compute_absolute_local_pos(&self) -> Result<AbsoluteLocalChunkPos, String> {
        let absolute_local_pos_base10x10 = match self.global_id_base10x10.last() {
            Some(absolute_local_pos_base10x10) => absolute_local_pos_base10x10.clone(),
            None => {
                return Err(
                    "Cannot compute absolute local position from chunk ID: Chunk ID is invalid.".to_string(),
                )
            }
        };

        return Ok(AbsoluteLocalChunkPos::from(absolute_local_pos_base10x10));
    }
}

// Module Functions
