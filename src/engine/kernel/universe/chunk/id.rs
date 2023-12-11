// Modules

// Local imports

// Internal imports
use crate::engine::kernel::math::*;

// External imports
use num_bigint::BigUint;
use std::hash::*;

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct LocalChunkIDBase10x10 {
    id: (u8, u8),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct LocalChunkIDBase10 {
    id: u8,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct LocalChunkID {
    id: u8,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GlobalChunkIDBase10x10 {
    id: Vec<(u8, u8)>,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GlobalChunkIDBase10 {
    id: BigUint,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GlobalChunkIDBase57 {
    id: String,
}

#[derive(Eq, Clone, Debug)]
pub struct ChunkID {
    parent_chunk_id: Option<Box<ChunkID>>,
    local_chunk_id: LocalChunkID,
    scale_index: u8,
}

// Implementations
impl Default for LocalChunkIDBase10x10 {
    fn default() -> Self {
        Self { id: (0, 0) }
    }
}

impl LocalChunkIDBase10x10 {
    pub fn new_from_tuple(id: (u8, u8)) -> Result<LocalChunkIDBase10x10, String> {
        if id.0 > 9 || id.1 > 9 {
            return Err("Cannot create local chunk ID Base10x10 from tuple: Tuple ID is too big.".to_string());
        }

        Ok(Self {
            id,
        })
    }

    pub fn get_id(&self) -> (u8, u8) {
        self.id
    }
}

impl Default for LocalChunkIDBase10 {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl LocalChunkIDBase10 {
    pub fn new_from_integer(id: u8) -> Result<LocalChunkIDBase10, String> {
        if id > 9 {
            return Err("Cannot create local chunk ID Base10 from integer: Integer ID is too big.".to_string());
        }

        Ok(Self {
            id,
        })
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }
}

impl Default for LocalChunkID {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl LocalChunkID {
    pub fn new_from_base10x10(id: LocalChunkIDBase10x10) -> LocalChunkID {
        let id = id.get_id();

        Self {
            id: id.0 * 10 + id.1,
        }
    }

    pub fn new_from_base10(id: LocalChunkIDBase10) -> LocalChunkID {
        let id = id.get_id();

        Self {
            id,
        }
    }

    pub fn get_base10x10(&self) -> (u8, u8) {
        (self.id / 10, self.id % 10)
    }

    pub fn get_base10(&self) -> u8 {
        self.id
    }
}

impl GlobalChunkIDBase10x10 {
    pub fn new_from_vec(id: Vec<(u8, u8)>) -> Result<GlobalChunkIDBase10x10, String> {
        if id.is_empty() {
            return Err("Cannot create global chunk ID Base10x10 from vector: Vector is empty.".to_string());
        }

        for id in &id {
            if id.0 > 9 || id.1 > 9 {
                return Err("Cannot create global chunk ID Base10x10 from vector: Vector ID element is too big.".to_string());
            }
        }

        Ok(Self {
            id,
        })
    }

    pub fn get_id(&self) -> &Vec<(u8, u8)> {
        &self.id
    }
}

impl GlobalChunkIDBase10 {
    pub fn new_from_biguint(id: BigUint) -> GlobalChunkIDBase10 {
        Self {
            id,
        }
    }

    pub fn get_id(&self) -> &BigUint {
        &self.id
    }
}

impl GlobalChunkIDBase57 {
    pub fn new_from_string(id: String) -> Result<GlobalChunkIDBase57, String> {
        if id.is_empty() {
            return Err("Cannot create global chunk ID Base57 from string: String is empty.".to_string());
        }

        Ok(Self {
            id,
        })
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}

impl TryFrom<Vec<(u8, u8)>> for ChunkID {
    type Error = String;

    fn try_from(mut chunk_id_base10x10: Vec<(u8, u8)>) -> Result<Self, Self::Error> {
        if chunk_id_base10x10.is_empty() {
            return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: Vector is empty."));
        }

        let chunk_id: ChunkID = match chunk_id_base10x10.remove(0).try_into() {
            Ok(parent_chunk_id) => parent_chunk_id,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
        };

        for local_chunk_id_base10x10 in chunk_id_base10x10 {
            let local_chunk_id = match LocalChunkID::new_from_base10x10(local_chunk_id_base10x10) {
                Ok(local_chunk_id) => local_chunk_id,
                Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
            };

            let next_chunk_id = match ChunkID::new(chunk_id, local_chunk_id) {
                Ok(chunk_id) => chunk_id,
                Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
            };

            let chunk_id = next_chunk_id;
        }

        Ok(chunk_id)
    }
}

impl TryInto<Vec<(u8, u8)>> for ChunkID {
    type Error = String;

    fn try_into(self) -> Result<Vec<(u8, u8)>, Self::Error> {
        let local_chunk_id_base10x10: (u8, u8) = match self.local_chunk_id.try_into() {
            Ok(local_chunk_id_base10x10) => local_chunk_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
        };

        if let Some(parent_chunk_id) = self.parent_chunk_id {
            let parent_chunk_id_base10x10: Vec<(u8, u8)> = match (*parent_chunk_id).try_into() {
                Ok(parent_chunk_id_base10x10) => parent_chunk_id_base10x10,
                Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
            };
            
            Ok([parent_chunk_id_base10x10, vec![local_chunk_id_base10x10]].concat())
        } else {
            Ok(vec![local_chunk_id_base10x10])
        }
    }
}

impl TryFrom<GlobalChunkIDBase10> for ChunkID {
    type Error = String;

    fn try_from(chunk_id_base10: GlobalChunkIDBase10) -> Result<Self, Self::Error> {
        let chunk_id_base10x10 = match BASE10X10_CONVERTER.convert_to_base10x10(chunk_id_base10.get_id().clone()) {
            Ok(global_id_base10x10) => global_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10 to chunk ID Base10x10: {}", e)),
        };

        let chunk_id: ChunkID = match chunk_id_base10x10.try_into() {
            Ok(chunk_id) => chunk_id,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
        };

        Ok(chunk_id)
    }
}

impl TryInto<GlobalChunkIDBase10> for ChunkID {
    type Error = String;

    fn try_into(self) -> Result<GlobalChunkIDBase10, Self::Error> {
        let chunk_id_base10x10: Vec<(u8, u8)> = match self.try_into() {
            Ok(chunk_id_base10x10) => chunk_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
        };

        let chunk_id_base10 = match BASE10X10_CONVERTER.convert_from_base10x10(chunk_id_base10x10) {
            Ok(global_id_base10) => global_id_base10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID Base10: {}", e)),
        };

        Ok(GlobalChunkIDBase10::new_from_biguint(chunk_id_base10))
    }
}

impl TryFrom<GlobalChunkIDBase57> for ChunkID {
    type Error = String;

    fn try_from(chunk_id_base57: GlobalChunkIDBase57) -> Result<Self, Self::Error> {
        let chunk_id_base10 = match BASE57_CONVERTER.convert_from_base57(chunk_id_base57.get_id().as_str()) {
            Ok(global_id_base10) => global_id_base10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base57 to chunk ID Base10: {}", e)),
        };

        let chunk_id_base10x10 = match BASE10X10_CONVERTER.convert_to_base10x10(chunk_id_base10) {
            Ok(global_id_base10x10) => global_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base57 to chunk ID Base10x10: {}", e)),
        };

        let chunk_id: ChunkID = match chunk_id_base10x10.try_into() {
            Ok(chunk_id) => chunk_id,
            Err(e) => return Err(format!("Cannot convert chunk ID Base57 to chunk ID: {}", e)),
        };

        Ok(chunk_id)
    }
}

impl TryInto<GlobalChunkIDBase57> for ChunkID {
    type Error = String;

    fn try_into(self) -> Result<GlobalChunkIDBase57, Self::Error> {
        let chunk_id_base10x10: Vec<(u8, u8)> = match self.try_into() {
            Ok(chunk_id_base10x10) => chunk_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
        };

        let chunk_id_base10 = match BASE10X10_CONVERTER.convert_from_base10x10(chunk_id_base10x10) {
            Ok(global_id_base10) => global_id_base10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID Base10: {}", e)),
        };

        let chunk_id_base57 = match BASE57_CONVERTER.convert_to_base57(chunk_id_base10) {
            Ok(global_id_base57) => global_id_base57,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10 to chunk ID Base57: {}", e)),
        };

        let chunk_id_base57 = match GlobalChunkIDBase57::new_from_string(chunk_id_base57) {
            Ok(global_id_base57) => global_id_base57,
            Err(e) => return Err(format!("Cannot convert chunk ID Base57 to chunk ID: {}", e)),
        };

        Ok(chunk_id_base57)
    }
}

impl From<LocalChunkID> for ChunkID {
    fn from(local_chunk_id: LocalChunkID) -> Self {
        ChunkID::new_root(local_chunk_id)
    }
}

impl TryInto<LocalChunkID> for ChunkID {
    type Error = String;

    fn try_into(self) -> Result<LocalChunkID, Self::Error> {
        if self.parent_chunk_id.is_some() {
            return Err("Cannot convert chunk ID to local chunk ID: Chunk ID is not a root chunk ID.".to_string());
        }

        let local_chunk_id = self.local_chunk_id;

        Ok(local_chunk_id)
    }
}

impl PartialEq for ChunkID {
    fn eq(&self, other: &Self) -> bool {
        let parent_ids_equal = match (&self.parent_chunk_id, &other.parent_chunk_id) {
            (Some(self_parent_chunk_id), Some(other_parent_chunk_id)) => self_parent_chunk_id == other_parent_chunk_id,
            (None, None) => true,
            _ => false,
        };

        parent_ids_equal && self.local_chunk_id == other.local_chunk_id
    }
}

impl Hash for ChunkID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent_chunk_id.hash(state);
        self.local_chunk_id.hash(state);
    }
}

impl Default for ChunkID {
    fn default() -> Self {
        Self {
            parent_chunk_id: None,
            local_chunk_id: LocalChunkID::default(),
            scale_index: 0,
        }
    }
}

impl ChunkID {
    pub fn new_root(local_chunk_id: LocalChunkID) -> ChunkID {
        ChunkID {
            parent_chunk_id: None,
            local_chunk_id,
            scale_index: 0,
        }
    }

    pub fn new(parent_chunk_id: ChunkID, local_chunk_id: LocalChunkID) -> Result<ChunkID, String> {
        if parent_chunk_id.scale_index == 63 {
            return Err("Cannot create chunk ID: Parent chunk has already reached the max scale index.".to_string());
        }

        Ok(ChunkID {
            parent_chunk_id: Some(Box::new(parent_chunk_id)),
            local_chunk_id,
            scale_index: parent_chunk_id.scale_index + 1,
        })
    }

    pub fn get_parent_chunk_id(&self) -> Option<&ChunkID> {
        match &self.parent_chunk_id {
            Some(parent_chunk_id) => Some(parent_chunk_id),
            None => None,
        }
    }

    pub fn get_local_chunk_id(&self) -> LocalChunkID {
        self.local_chunk_id
    }

    pub fn get_scale_index(&self) -> u8 {
        self.scale_index
    }
}

// Module Functions
