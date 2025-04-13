use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
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
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| {
                            let mut commands = main_access.commands;

                            commands.spawn((
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
