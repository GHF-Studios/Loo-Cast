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
pub struct GlobalUniverseID {
    id: u32
}

// Implementations
impl GlobalUniverseID {
    pub fn new(id: u32) -> Self {
        Self {
            id
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

// Module Functions
