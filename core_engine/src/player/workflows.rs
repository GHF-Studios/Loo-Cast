use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Player",
    workflows: [
        SpawnPlayer {
            user_imports: {
                use bevy::prelude::{Commands, Entity, Query, Res, ResMut};

                use crate::{
                    player::bundles::PlayerBundle,
                    player::components::Player,
                    follower::components::FollowerTarget,
                };
            },
            user_items: {},
            stages: [
                ValidateAndSpawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            player_query: Query<'w, 's, &'static Player>,
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
                            let mut commands = main_access.commands;
                            let player_query = main_access.player_query;

                            if !player_query.is_empty() {
                                return Err(Error::PlayerAlreadySpawned);
                            }

                            let player_bundle = PlayerBundle::default();
                            let player_entity = player_bundle.chunk_loader().chunk_owner_id().entity();

                            commands.entity(player_entity).insert((
                                player_bundle,
                                FollowerTarget {
                                    id: "main_camera".to_string(),
                                },
                            ));
                            bevy::prelude::debug!("Spawned player entity: {:?}", player_entity);

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

        DespawnPlayer {
            user_imports: {
                use bevy::prelude::{Commands, Entity, Query, Res, ResMut, debug, DespawnRecursiveExt};

                use crate::{
                    chunk_loader::components::ChunkLoader,
                    player::bundles::PlayerBundle,
                    player::components::Player,
                    follower::components::FollowerTarget,
                    utils::DropHook,
                };
            },
            user_items: {},
            stages: [
                ValidateAndDespawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            chunk_loader_with_drop_hook_query: Query<'w, 's, Entity, (With<Player>, Without<DropHook<ChunkLoader>>)>,
                            chunk_loader_without_drop_hook_query: Query<'w, 's, Entity, (With<Player>, With<DropHook<ChunkLoader>>)>,
                        }
                        struct State {}
                        enum Error {
                            PlayerAlreadyMarkedForDespawn,
                            PlayerAlreadyDespawned,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> Result<State, Error> {
                            let mut commands = main_access.commands;
                            let chunk_loader_with_drop_hook_query = main_access.chunk_loader_with_drop_hook_query;
                            let chunk_loader_without_drop_hook_query = main_access.chunk_loader_without_drop_hook_query;

                            match (chunk_loader_with_drop_hook_query.get_single().is_err(), chunk_loader_without_drop_hook_query.get_single().is_err()) {
                                (true, true) => { unreachable!() },
                                (true, false) => {},
                                (false, true) => {
                                    return Err(Error::PlayerAlreadyMarkedForDespawn);
                                },
                                (false, false) => {
                                    return Err(Error::PlayerAlreadyDespawned);
                                }
                            }

                            let player_entity = chunk_loader_without_drop_hook_query.single();
                            commands.entity(player_entity).insert(DropHook::<ChunkLoader>::default());
                            debug!("Marked player entity for despawning");

                            Ok(State {})
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, ()>, Error> {
                            // Despawn the player entity if the DropHook<ChunkLoader> marker has been removed, else wait

                            let mut commands = main_access.commands;
                            let chunk_loader_with_drop_hook_query = main_access.chunk_loader_with_drop_hook_query;
                            let chunk_loader_without_drop_hook_query = main_access.chunk_loader_without_drop_hook_query;

                            match (chunk_loader_with_drop_hook_query.get_single().is_err(), chunk_loader_without_drop_hook_query.get_single().is_err()) {
                                (true, true) => { unreachable!() },
                                (true, false) => {},
                                (false, true) => return Ok(Wait(State {})),
                                (false, false) => {
                                    return Err(Error::PlayerAlreadyDespawned);
                                }
                            }

                            let player_entity = chunk_loader_without_drop_hook_query.single();
                            commands.entity(player_entity).despawn_recursive();
                            debug!("Despawned player entity: {:?}", player_entity);

                            Ok(Done(()))
                        }
                    ]
                }
            ],
        }
    ]
}
