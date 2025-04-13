pub mod validate_and_spawn;

use spacetime_engine_macros::define_worfklow_stages;

define_worfklow_stages![
    stage!(Ecs, "ValidateAndSpawn")
];