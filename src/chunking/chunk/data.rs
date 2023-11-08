// Modules


// Local imports


// Internal imports


// External imports


// Static variables


// Constant variables


// Types


// Enums


// Structs
pub struct ChunkData {
    placeholder_data: Option<i32>,
}

// Implementations
impl ChunkData {
    pub fn new() -> ChunkData {
        ChunkData {
            placeholder_data: None,
        }
    }

    pub fn get_placeholder_data(&self) -> Option<i32> {
        return self.placeholder_data;
    }

    pub fn set_placeholder_data(&mut self, placeholder_data: Option<i32>) {
        self.placeholder_data = placeholder_data;
    }
}

// Module Functions
