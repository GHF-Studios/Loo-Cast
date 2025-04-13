pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    pub use bevy::prelude::*;

    pub use crate::chunk::{components::ChunkComponent, resources::ChunkManager};
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("TransferChunkOwnership", [
    stage!(Ecs, "FindAndTransferOwnership")
]);