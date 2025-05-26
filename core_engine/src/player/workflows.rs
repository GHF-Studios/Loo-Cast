use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Player",
    workflows: [
        SpawnPlayer {
            user_imports: {
                use bevy::prelude::{Commands, Entity, Query, Res, ResMut};

                use crate::{
                    player::bundles::PlayerBundle,
                    player::components::PlayerComponent,
                    follower::components::FollowerTargetComponent,
                };
            },
            user_items: {},
            stages: [
                ValidateAndSpawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            player_query: Query<'w, 's, &'static PlayerComponent>,
                        }
                        struct State {
                            player_entity: Entity,
                        }
                        struct Output {
                            player_entity: Entity,
                        }
                        enum Error {
                            PlayerAlreadySpawned
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> Result<State, Error> {
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

                            Ok(State { player_entity })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let mut commands = main_access.commands;

                            if commands.get_entity(state.player_entity).is_some() {
                                Ok(Outcome::Done(Output { player_entity: state.player_entity }))
                            } else {
                                Ok(Outcome::Wait(state))
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
