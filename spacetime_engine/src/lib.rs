// Modules
pub mod kernel;
pub mod system;

// Local imports

// Internal imports

// External imports
use bevy::app::*;
use std::any::TypeId;

// Static variables

// Constant variables

// Traits
pub trait Mod {
    fn id(&self) -> TypeId;
    fn dependencies(&self) -> Vec<TypeId>;
    fn register_mod(&self, app: &mut App);
}

// Types

// Enums

// Structs

// Implementations

// Module Functions
