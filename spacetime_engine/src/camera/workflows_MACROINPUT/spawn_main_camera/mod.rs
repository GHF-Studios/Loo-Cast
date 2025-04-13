pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    pub use bevy::prelude::*;

    pub use crate::camera::components::MainCamera;
    pub use crate::config::statics::CONFIG;
    pub use crate::follower::components::{FollowerComponent, FollowerTargetComponent};
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("SpawnMainCamera", [
    stage!(Ecs, "ValidateAndSpawn")
]);