// Modules

// Local imports

// Internal imports

// External imports

// Static variables

// Constant variables

// Types

// Enums

// Structs
pub struct Math;

// Implementations
impl Math {
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }
}

// Module Functions
