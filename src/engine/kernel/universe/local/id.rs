// Modules

// Local imports

// Internal imports

// External imports

// Static variables

// Constant variables

// Types

// Enums

// Structs
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalUniverseID {
    id: u32,
}

// Implementations
impl Default for LocalUniverseID {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl LocalUniverseID {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

// Module Functions
