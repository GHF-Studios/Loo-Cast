use spacetime_engine_macros::define_workflow_mod;

define_workflow_mod! {
    name: "Player",
    workflows: [
        SpawnPlayer {
            user_imports: {
                use crate::{
                    player::bundles::PlayerBundle,
                    follower::components::FollowerTargetComponent,
                };
            },
            user_items: {},
            stages: [
                ValidateAndSpawn: Ecs {
                    core_types: [],
                    core_functions: [
                        fn RunEcs |world| {
                            world.spawn((
                                PlayerBundle::default(),
                                FollowerTargetComponent {
                                    id: "main_camera".to_string(),
                                },
                            ));
                        }
                    ]
                }
            ]
        }
    ]
}
