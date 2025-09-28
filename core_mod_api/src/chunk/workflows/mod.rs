pub mod external;

use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Chunk",
    workflows: [
        SpawnChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;

                use crate::chunk::workflows::external::spawn_chunks::{
                    MainAccess as ValidateAndSpawnAndWaitMainAccess,
                    Input as ValidateAndSpawnAndWaitInput,
                    State as ValidateAndSpawnAndWaitState,
                    Output as ValidateAndSpawnAndWaitOutput,
                    Error as ValidateAndSpawnAndWaitError,
                    setup_ecs_while as validate_and_spawn_and_wait_setup_ecs_while,
                    run_ecs_while as validate_and_spawn_and_wait_run_ecs_while,
                };
                use crate::usf::scale::*;
                use crate::utils::progress::Progress;
            },
            user_items: {
            },
            stages: [
                ValidateAndSpawnAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: ValidateAndSpawnAndWaitMainAccess<'w, 's>,
                        }

                        struct Input {
                            inner: ValidateAndSpawnAndWaitInput,
                        }
                        struct State {
                            inner: Progress<ValidateAndSpawnAndWaitState, ValidateAndSpawnAndWaitOutput>,
                        }
                        struct Output {
                            inner: ValidateAndSpawnAndWaitOutput,
                        }
                        enum Error {
                            Inner(ValidateAndSpawnAndWaitError),
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let state = validate_and_spawn_and_wait_setup_ecs_while(input.inner, main_access.inner).map_err(|e| Error::Inner(e))?;
                            
                            Ok(State {
                                inner: Progress::Unfinished(state),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress = match state.inner {
                                Progress::Unfinished(state) => validate_and_spawn_and_wait_run_ecs_while(state, main_access.inner).map_err(|e| Error::Inner(e))?.into_progress(),
                                Progress::Finished(output) => Progress::Finished(output)
                            };
                            
                            if progress.is_finished() {
                                return Ok(Done(Output {
                                    inner: progress.unwrap_finished(),
                                }));
                            }
                            
                            Ok(Wait(State {
                                inner: progress,
                            }))
                        }
                    ]
                }
            ]
        }

        DespawnChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;

                use crate::chunk::workflows::external::despawn_chunks::{
                    MainAccess as FindAndDespawnAndWaitMainAccess,
                    Input as FindAndDespawnAndWaitInput,
                    State as FindAndDespawnAndWaitState,
                    Output as FindAndDespawnAndWaitOutput,
                    Error as FindAndDespawnAndWaitError,
                    setup_ecs_while as find_and_despawn_and_wait_setup_ecs_while,
                    run_ecs_while as find_and_despawn_and_wait_run_ecs_while,
                };
                use crate::usf::scale::*;
                use crate::utils::progress::Progress;
            },
            user_items: {
            },
            stages: [
                FindAndDespawnAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: FindAndDespawnAndWaitMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: FindAndDespawnAndWaitInput,
                        }
                        struct State {
                            inner: Progress<FindAndDespawnAndWaitState, FindAndDespawnAndWaitOutput>,
                        }
                        struct Output {
                            inner: FindAndDespawnAndWaitOutput,
                        }

                        enum Error {
                            Inner(FindAndDespawnAndWaitError),
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let state = find_and_despawn_and_wait_setup_ecs_while(input.inner, main_access.inner).map_err(|e| Error::Inner(e))?;
                            
                            Ok(State {
                                inner: Progress::Unfinished(state),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress = match state.inner {
                                Progress::Unfinished(state) => find_and_despawn_and_wait_run_ecs_while(state, main_access.inner).map_err(|e| Error::Inner(e))?.into_progress(),
                                Progress::Finished(output) => Progress::Finished(output)
                            };
                            
                            if progress.is_finished() {
                                return Ok(Done(Output {
                                    inner: progress.unwrap_finished(),
                                }));
                            }

                            Ok(Wait(State {
                                inner: progress,
                            }))
                        }
                    ]
                }
            ]
        }

        TransferChunkOwnerships, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;

                use crate::chunk::workflows::external::transfer_chunk_ownerships::{
                    MainAccess as FindAndTransferOwnershipMainAccess,
                    Input as FindAndTransferOwnershipInput,
                    Output as FindAndTransferOwnershipOutput,
                    Error as FindAndTransferOwnershipError,
                    run_ecs as find_and_transfer_ownership_run_ecs,
                };
                use crate::usf::scale::*;
                use crate::utils::progress::Progress;
            },
            user_items: {
            },
            stages: [
                FindAndTransferOwnership: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: FindAndTransferOwnershipMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: FindAndTransferOwnershipInput,
                        }
                        struct Output {
                            inner: FindAndTransferOwnershipOutput,
                        }
                        enum Error {
                            Inner(FindAndTransferOwnershipError),
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<Output, Error> {
                            let output = find_and_transfer_ownership_run_ecs(input.inner, main_access.inner).map_err(|e| Error::Inner(e))?;

                            Ok(Output {
                                inner: output,
                            })
                        }
                    ]
                }
            ]
        }
    ]
}
