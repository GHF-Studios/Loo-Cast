pub mod spawn_main_camera;

use spacetime_engine_macros::define_workflow_mod;

define_workflow_mod!("Camera", [
    workflow!("SpawnMainCamera")
]);