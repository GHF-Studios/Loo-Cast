pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    use crate::{
        player::bundles::PlayerBundle,
        follower::components::FollowerTargetComponent,
    };
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("SpawnPlayer", [
    stage!(Ecs, "ValidateAndSpawn")
]);