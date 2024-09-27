// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod hooks;
pub mod systems;
pub mod utilities;

// Miscelaneous
pub mod singletons;
pub mod traits;

use bevy::prelude::*;

pub(in crate) struct OperationsPlugin;
impl Plugin for OperationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::startup)
            .add_systems(PostUpdate, systems::post_update);
    }
}

// TODO:    Make it so every registry and type registry is rooted at the module level, not at the root library layer.
        //  This is to not have to go through a single arc mutex singleton to access about everything that's accessible, but have seperate singletons for each module,
        //  so we can selectively lock either the entire library, or just parts of it to allow for parallelism.
// TODO: MAYBE: Implement a way to easily request operations for the operation queue, and to easily request data from the main type registry
// TODO: Implement operations and hooks for all types
    // TODO: Zeroary: Figure out a way to make a ChunkPosition the Key to the serialized data (see ChatGPT)
    // TODO: Primary: Implement saving/loading operations for chunks, where the serialized chunk and it's contents are stored in memory, instead of on disk (for now)
    // TODO: Secondary: Implement any additional operations (and potentially hooks) which may be useful (like changing the owner of a chunk, or the owner of a chunk actor, or the load radius of a chunk loader, for example)
    // TODO: Tertiary: Extend to 'Camera', 'Player', 'Follower', and 'Physics', essentially reworking the entire code base; I guess; framework richie go brr)
// TODO: Integrate and Implement operations module into existing modules, and bundle that operation-related code in an 'operations' sub-module for each existing module, and like essentially finish up the code base rework

