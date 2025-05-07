use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Player",
    workflows: [
        SpawnPlayer {
            user_imports: {
                use crate::{
                    player::bundles::PlayerBundle,
                    player::components::PlayerComponent,
                    follower::components::FollowerTargetComponent,
                };
            },
            user_items: {},
            stages: [
                ValidateAndSpawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            player_query: Query<'w, 's, &'static PlayerComponent>,
                        }
                        struct Output {
                            player_entity: Entity,
                        }
                        enum Error {
                            PlayerAlreadySpawned
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Result<Output, Error> {
                            bevy::prelude::debug!("Spawning player..");
                            let mut commands = main_access.commands;
                            let player_query = main_access.player_query;

                            if !player_query.is_empty() {
                                return Err(Error::PlayerAlreadySpawned);
                            }

                            let player_entity = commands.spawn((
                                PlayerBundle::default(),
                                FollowerTargetComponent {
                                    id: "main_camera".to_string(),
                                },
                            )).id();

                            Ok(Output { player_entity })
                        }
                    ]
                }
            ]
        }
    ]
}
