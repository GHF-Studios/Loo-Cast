pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    pub use bevy::prelude::*;

    pub use crate::chunk::{components::ChunkComponent, resources::ChunkManager, functions::chunk_pos_to_world};
    pub use crate::config::statics::CONFIG;
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("SpawnChunk", [
    stage!(Ecs, "ValidateAndSpawn")
]);