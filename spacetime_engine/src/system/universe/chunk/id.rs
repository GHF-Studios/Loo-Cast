// Modules

// Local imports

// Internal imports

// External imports
use lazy_static::*;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use std::hash::*;

// Static variables
lazy_static! {
    static ref BASE10X10_CONVERTER: GlobalChunkIDBase10x10Converter = GlobalChunkIDBase10x10Converter::new();
    static ref BASE57_CONVERTER: GlobalChunkIDBase57Converter = GlobalChunkIDBase57Converter::new();
}

// Constant variables

// Types

// Enums

// Structs
struct GlobalChunkIDBase10x10Converter {
    max_digits: usize,
    power_sums: Vec<BigUint>,
    offsets: Vec<BigUint>,
}

struct GlobalChunkIDBase57Converter {
    max_digits: usize,
}

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
impl GlobalChunkIDBase10x10Converter {
    pub fn new() -> Self {
        let max_digits = 64;
        let mut power_sums = Vec::with_capacity(max_digits);
        let mut offsets = Vec::with_capacity(max_digits);
        let mut power_sum = BigUint::from(0u32);

        for i in 1..=max_digits {
            power_sum += BigUint::from(100u32).pow(i as u32);

            power_sums.push(power_sum.clone());

            offsets.push(BigUint::from(100u32).pow(i as u32));
        }

        GlobalChunkIDBase10x10Converter {
            max_digits,
            power_sums,
            offsets,
        }
    }

    pub fn convert_to_base10x10(&self, mut input: GlobalChunkIDBase10) -> Result<GlobalChunkIDBase10x10, String> {
        let mut input = input.id;
        let mut expected_pairs = 1u32;

        for sum in &self.power_sums {
            if input >= *sum {
                expected_pairs += 1u32;
            } else {
                break;
            }
        }
        if expected_pairs > self.max_digits as u32 {
            return Err(
                "Base10 input is too large for the specified max number of base10x10 digits!"
                    .to_string(),
            );
        }

        for offset in &self.offsets {
            if input >= *offset {
                input -= offset.clone();
            }
        }

        let mut input_digits = input.to_radix_le(10);

        input_digits.reverse();

        let mut input_digit_pairs = Vec::new();

        while !input_digits.is_empty() {
            if input_digits.len() == 1 {
                if let Some(second) = input_digits.pop() {
                    let first = 0u8;

                    input_digit_pairs.push((first, second));
                } else {
                    unreachable!();
                }
                break;
            } else if let Some(second) = input_digits.pop() {
                if let Some(first) = input_digits.pop() {
                    input_digit_pairs.push((first, second));

                    continue;
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }
        }

        while input_digit_pairs.len() < expected_pairs as usize {
            input_digit_pairs.push((0, 0));
        }

        input_digit_pairs.reverse();

        let output = match GlobalChunkIDBase10x10::new_from_vec(input_digit_pairs) {
            Ok(output) => output,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10 to chunk ID Base10x10: {}", e)),
        };

        Ok(output)
    }

    pub fn convert_from_base10x10(&self, input: GlobalChunkIDBase10x10) -> Result<GlobalChunkIDBase10, String> {
        let mut input = input.id;
        
        if input.len() > self.max_digits {
            return Err("Base10x10 input has more pairs than allowed by the specified max number of base10x10 digits!".to_string());
        }

        if input.is_empty() {
            return Err("Base10x10 input is empty!".to_string());
        }

        let mut result = BigUint::zero();
        let mut num_pairs = 0;

        for (i, &(first, second)) in input.iter().rev().enumerate() {
            if first >= 10 || second >= 10 {
                return Err("Invalid digits in base10x10 input!".to_string());
            }

            let pair_value = BigUint::from((first as u32) * 10 + (second as u32));

            result += pair_value * BigUint::from(100u32).pow(i as u32);

            num_pairs += 1;
        }

        for i in 0..num_pairs - 1 {
            result += self.offsets[i].clone();
        }

        let output = GlobalChunkIDBase10::new_from_biguint(result);

        Ok(output)
    }
}

impl GlobalChunkIDBase57Converter {
    pub fn new() -> Self {
        let max_digits = 73;

        GlobalChunkIDBase57Converter { max_digits }
    }

    pub fn convert_to_base57(&self, mut input: GlobalChunkIDBase10) -> Result<GlobalChunkIDBase57, String> {
        let mut input = input.id;
        let charset = "abcdefghijklmnopqrstuvwxyz0123456789+,;_-'~`´@!$%&()[]{}=";
        let base = BigUint::from(57u32);
        let mut result = String::new();

        while input != BigUint::zero() {
            let rem = &input % &base;

            if let Some(character) = charset.chars().nth(
                rem.to_usize().ok_or(
                    "Base10 input is too large for the specified max number of base57 digits!"
                        .to_string(),
                )?,
            ) {
                result.push(character);
            }

            input /= &base;
        }

        while result.chars().count() < self.max_digits {
            if let Some(character) = charset.chars().next() {
                result.push(character);
            } else {
                unreachable!();
            }
        }

        if result.chars().count() > self.max_digits {
            unreachable!();
        }

        result = result.chars().rev().collect::<String>();

        let output = match GlobalChunkIDBase57::new_from_string(result) {
            Ok(output) => output,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10 to chunk ID Base57: {}", e)),
        };

        Ok(output)
    }

    pub fn convert_from_base57(&self, input: GlobalChunkIDBase57) -> Result<GlobalChunkIDBase10, String> {
        let input = input.id;
        
        if input.chars().count() > self.max_digits {
            return Err(
                "Base57 input is too large for the specified max number of base57 digits!"
                    .to_string(),
            );
        }

        let charset = "abcdefghijklmnopqrstuvwxyz0123456789+,;_-'~`´@!$%&()[]{}=";
        let base = BigUint::from(57u32);
        let mut result = BigUint::zero();
        let mut multiplier = BigUint::one();

        for char in input.chars().rev() {
            if let Some(position) = charset.chars().position(|c| c == char) {
                let value = BigUint::from(
                    position
                        .to_u32()
                        .ok_or("An unexpected error occured!".to_string())?,
                );

                result += value * &multiplier;
            } else {
                return Err(format!("Invalid digit '{}' in the base57 input!", char));
            }

            multiplier *= &base;
        }

        let output = GlobalChunkIDBase10::new_from_biguint(result);

        Ok(output)
    }
}

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

    pub fn get_base10x10(&self) -> Result<LocalChunkIDBase10x10, String> {
        let id = self.id;

        let id = (id / 10, id % 10);

        let id = match LocalChunkIDBase10x10::new_from_tuple(id) {
            Ok(id) => id,
            Err(e) => return Err(format!("Cannot convert local chunk ID to local chunk ID Base10x10: {}", e)),
        };

        Ok(id)
    }

    pub fn get_base10(&self) -> Result<LocalChunkIDBase10, String> {
        let id = self.id;

        let id = match LocalChunkIDBase10::new_from_integer(id) {
            Ok(id) => id,
            Err(e) => return Err(format!("Cannot convert local chunk ID to local chunk ID Base10: {}", e)),
        };

        Ok(id)
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

impl TryFrom<GlobalChunkIDBase10x10> for ChunkID {
    type Error = String;

    fn try_from(mut chunk_id_base10x10: GlobalChunkIDBase10x10) -> Result<Self, Self::Error> {
        let chunk_id_base10x10 = chunk_id_base10x10.get_id();
        if chunk_id_base10x10.is_empty() {
            return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: Vector is empty."));
        }

        let mut chunk_id_base10x10 = chunk_id_base10x10.clone();

        let local_chunk_id_base10x10 = match LocalChunkIDBase10x10::new_from_tuple(chunk_id_base10x10.remove(0)) {
            Ok(local_chunk_id_base10x10) => local_chunk_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
        };

        let local_chunk_id = LocalChunkID::new_from_base10x10(local_chunk_id_base10x10);

        let chunk_id: ChunkID = match local_chunk_id.try_into() {
            Ok(chunk_id) => chunk_id,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
        };

        for chunk_id_base10x10_part in chunk_id_base10x10 {
            let local_chunk_id_base10x10 = match LocalChunkIDBase10x10::new_from_tuple(chunk_id_base10x10_part) {
                Ok(local_chunk_id_base10x10) => local_chunk_id_base10x10,
                Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
            };

            let local_chunk_id = LocalChunkID::new_from_base10x10(local_chunk_id_base10x10);

            let next_chunk_id: ChunkID = match local_chunk_id.try_into() {
                Ok(chunk_id) => chunk_id,
                Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID: {}", e)),
            };

            let chunk_id = next_chunk_id;
        }

        Ok(chunk_id)
    }
}

impl TryInto<GlobalChunkIDBase10x10> for ChunkID {
    type Error = String;

    fn try_into(self) -> Result<GlobalChunkIDBase10x10, Self::Error> {
        let local_chunk_id_base10x10 = match self.local_chunk_id.get_base10x10() {
            Ok(local_chunk_id_base10x10) => local_chunk_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
        };

        if let Some(parent_chunk_id) = self.parent_chunk_id {
            let parent_chunk_id_base10x10: GlobalChunkIDBase10x10 = match (*parent_chunk_id).try_into() {
                Ok(parent_chunk_id_base10x10) => parent_chunk_id_base10x10,
                Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
            };
            
            let chunk_id_base10x10 = [parent_chunk_id_base10x10.get_id().clone(), vec![local_chunk_id_base10x10.get_id()]].concat();

            let chunk_id_base10x10 = match GlobalChunkIDBase10x10::new_from_vec(chunk_id_base10x10) {
                Ok(chunk_id_base10x10) => chunk_id_base10x10,
                Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
            };

            Ok(chunk_id_base10x10)
        } else {
            let chunk_id_base10x10 = vec![local_chunk_id_base10x10.get_id()];

            let chunk_id_base10x10 = match GlobalChunkIDBase10x10::new_from_vec(chunk_id_base10x10) {
                Ok(chunk_id_base10x10) => chunk_id_base10x10,
                Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
            };

            Ok(chunk_id_base10x10)
        }
    }
}

impl TryFrom<GlobalChunkIDBase10> for ChunkID {
    type Error = String;

    fn try_from(chunk_id_base10: GlobalChunkIDBase10) -> Result<Self, Self::Error> {
        let chunk_id_base10x10 = match BASE10X10_CONVERTER.convert_to_base10x10(chunk_id_base10) {
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
        let chunk_id_base10x10: GlobalChunkIDBase10x10 = match self.try_into() {
            Ok(chunk_id_base10x10) => chunk_id_base10x10,
            Err(e) => return Err(format!("Cannot convert chunk ID to chunk ID Base10x10: {}", e)),
        };

        let chunk_id_base10 = match BASE10X10_CONVERTER.convert_from_base10x10(chunk_id_base10x10) {
            Ok(global_id_base10) => global_id_base10,
            Err(e) => return Err(format!("Cannot convert chunk ID Base10x10 to chunk ID Base10: {}", e)),
        };

        Ok(chunk_id_base10)
    }
}

impl TryFrom<GlobalChunkIDBase57> for ChunkID {
    type Error = String;

    fn try_from(chunk_id_base57: GlobalChunkIDBase57) -> Result<Self, Self::Error> {
        let chunk_id_base10 = match BASE57_CONVERTER.convert_from_base57(chunk_id_base57) {
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
        let chunk_id_base10x10: GlobalChunkIDBase10x10 = match self.try_into() {
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
