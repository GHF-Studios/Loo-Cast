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
                            inner_scale_quecto_meter_000001: ValidateAndSpawnAndWaitMainAccess<'w, 's, ScaleQuectoMeter000001>,
                        }

                        struct Input {
                            inner_scale_quecto_meter_000001: ValidateAndSpawnAndWaitInput<ScaleQuectoMeter000001>,
                        }
                        struct State {
                            inner_scale_quecto_meter_000001: Progress<ValidateAndSpawnAndWaitState, ValidateAndSpawnAndWaitOutput>,
                        }
                        struct Output {
                            inner_scale_quecto_meter_000001: ValidateAndSpawnAndWaitOutput,
                        }
                        enum Error {
                            ScaleQuectoMeter000001Error(ValidateAndSpawnAndWaitError),
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let state_scale_quecto_meter_000001 = validate_and_spawn_and_wait_setup_ecs_while(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001).map_err(|e| Error::ScaleQuectoMeter000001Error(e))?;
                            
                            Ok(State {
                                inner_scale_quecto_meter_000001: Progress::Unfinished(state_scale_quecto_meter_000001),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress_scale_quecto_meter_000001 = match state.inner_scale_quecto_meter_000001 {
                                Progress::Unfinished(state) => validate_and_spawn_and_wait_run_ecs_while(state, main_access.inner_scale_quecto_meter_000001).map_err(|e| Error::ScaleQuectoMeter000001Error(e))?.into_progress(),
                                Progress::Finished(output) => Progress::Finished(output)
                            };
                            
                            if progress_scale_quecto_meter_000001.is_finished() {
                                return Ok(Done(Output {
                                    inner_scale_quecto_meter_000001: progress_scale_quecto_meter_000001.unwrap_finished(),
                                }));
                            }
                            
                            Ok(Wait(State {
                                inner_scale_quecto_meter_000001: progress_scale_quecto_meter_000001,
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
                            inner_scale_quecto_meter_000001: FindAndDespawnAndWaitMainAccess<'w, 's, ScaleQuectoMeter000001>,
                        }
                        struct Input {
                            inner_scale_quecto_meter_000001: FindAndDespawnAndWaitInput,
                        }
                        struct State {
                            inner_scale_quecto_meter_000001: Progress<FindAndDespawnAndWaitState, FindAndDespawnAndWaitOutput>,
                        }
                        struct Output {
                            inner_scale_quecto_meter_000001: FindAndDespawnAndWaitOutput,
                        }

                        enum Error {
                            ScaleQuectoMeter000001Error(FindAndDespawnAndWaitError),
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let state_scale_quecto_meter_000001 = find_and_despawn_and_wait_setup_ecs_while(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001).map_err(|e| Error::ScaleQuectoMeter000001Error(e))?;
                            
                            Ok(State {
                                inner_scale_quecto_meter_000001: Progress::Unfinished(state_scale_quecto_meter_000001),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress_scale_quecto_meter_000001 = match state.inner_scale_quecto_meter_000001 {
                                Progress::Unfinished(state) => find_and_despawn_and_wait_run_ecs_while(state, main_access.inner_scale_quecto_meter_000001).map_err(|e| Error::ScaleQuectoMeter000001Error(e))?.into_progress(),
                                Progress::Finished(output) => Progress::Finished(output)
                            };
                            
                            if progress_scale_quecto_meter_000001.is_finished() {
                                return Ok(Done(Output {
                                    inner_scale_quecto_meter_000001: progress_scale_quecto_meter_000001.unwrap_finished(),
                                }));
                            }

                            Ok(Wait(State {
                                inner_scale_quecto_meter_000001: progress_scale_quecto_meter_000001,
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
                            inner_scale_quecto_meter_000001: FindAndTransferOwnershipMainAccess<'w, 's, ScaleQuectoMeter000001>,
                        }
                        struct Input {
                            inner_scale_quecto_meter_000001: FindAndTransferOwnershipInput<ScaleQuectoMeter000001>,
                        }
                        struct Output {
                            inner_scale_quecto_meter_000001: FindAndTransferOwnershipOutput,
                        }
                        enum Error {
                            ScaleQuectoMeter000001Error(FindAndTransferOwnershipError),
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<Output, Error> {
                            let output_scale_quecto_meter_000001 = find_and_transfer_ownership_run_ecs(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001).map_err(|e| Error::ScaleQuectoMeter000001Error(e))?;

                            Ok(Output {
                                inner_scale_quecto_meter_000001: output_scale_quecto_meter_000001,
                            })
                        }
                    ]
                }
            ]
        }
    ]
}
