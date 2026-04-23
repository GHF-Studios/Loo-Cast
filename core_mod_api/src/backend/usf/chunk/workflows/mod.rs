pub mod external;

use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "UsfChunk",
    root_path: usf::chunk,
    workflows: [
        SpawnChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::bevy::prelude::ResMut;

                use crate::usf::chunk::workflows::external::spawn_chunks::{
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
                            let state = validate_and_spawn_and_wait_setup_ecs_while(input.inner, main_access.inner).map_err(Error::Inner)?;

                            Ok(State {
                                inner: Progress::Unfinished(state),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress = match state.inner {
                                Progress::Unfinished(state) => validate_and_spawn_and_wait_run_ecs_while(state, main_access.inner).map_err(Error::Inner)?.into_progress(),
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
                use crate::bevy::prelude::ResMut;

                use crate::usf::chunk::workflows::external::despawn_chunks::{
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
                            let state = find_and_despawn_and_wait_setup_ecs_while(input.inner, main_access.inner).map_err(Error::Inner)?;

                            Ok(State {
                                inner: Progress::Unfinished(state),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress = match state.inner {
                                Progress::Unfinished(state) => find_and_despawn_and_wait_run_ecs_while(state, main_access.inner).map_err(Error::Inner)?.into_progress(),
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

        ReconcileChunkRealizationArtifacts, timeout_secs: 5.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::bevy::prelude::ResMut;
                use crate::usf::chunk::realization::reconcile_workflow::{
                    CommitOutput as ApplyOutputsOutput,
                    Error as ReconcileChunkRealizationArtifactsError,
                    Input as ResolveIntentsInput,
                    MainAccess as ApplyOutputsMainAccess,
                    ResolvedIntentsOutput,
                    State as ApplyOutputsState,
                    run_async as run_resolve_intents_async,
                    run_ecs_while as run_apply_outputs_ecs_while,
                    setup_ecs_while as setup_apply_outputs_ecs_while,
                };
                use crate::utils::progress::Progress;
            },
            user_items: {
            },
            stages: [
                ResolveIntents: Async, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct Input {
                            inner: ResolveIntentsInput,
                        }
                        struct Output {
                            inner: ResolvedIntentsOutput,
                        }
                        enum Error {
                            Inner(ReconcileChunkRealizationArtifactsError),
                        }
                    ],
                    core_functions: [
                        fn RunAsync |input| -> Result<Output, Error> {
                            let output = run_resolve_intents_async(input.inner).map_err(Error::Inner)?;
                            Ok(Output {
                                inner: output,
                            })
                        }
                    ]
                },

                ApplyOutputs: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: ApplyOutputsMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: ResolvedIntentsOutput,
                        }
                        struct State {
                            inner: Progress<ApplyOutputsState, ApplyOutputsOutput>,
                        }
                        struct Output {
                            inner: ApplyOutputsOutput,
                        }
                        enum Error {
                            Inner(ReconcileChunkRealizationArtifactsError),
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let state = setup_apply_outputs_ecs_while(input.inner, main_access.inner).map_err(Error::Inner)?;

                            Ok(State {
                                inner: Progress::Unfinished(state),
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let progress = match state.inner {
                                Progress::Unfinished(state) => {
                                    run_apply_outputs_ecs_while(state, main_access.inner).map_err(Error::Inner)?.into_progress()
                                }
                                Progress::Finished(output) => Progress::Finished(output),
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
    ]
}
