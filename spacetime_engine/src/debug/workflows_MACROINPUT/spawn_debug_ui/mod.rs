pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    use iyes_perf_ui::{
        entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
        prelude::{PerfUiEntryEntityCount, PerfUiRoot},
    };
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("SpawnDebugUI", [
    stage!(Ecs, "ValidateAndSpawn")
]);