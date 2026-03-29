pub mod external;

use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Chunk",
    workflows: [
        SpawnChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::bevy::prelude::ResMut;

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

        HydrateChunkVisuals, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::bevy::prelude::{Commands, ResMut};
                use crate::chunk::workflows::external::hydrate_chunk_visuals::{
                    ArtifactsOutput as BuildArtifactsOutput,
                    CommitOutput as CommitArtifactsOutput,
                    Error as HydrateChunkVisualsError,
                    Input as BuildArtifactsInput,
                    MainAccess as CommitArtifactsMainAccess,
                    run_async as run_build_artifacts_async,
                    run_ecs as run_commit_artifacts_ecs,
                };
            },
            user_items: {
            },
            stages: [
                BuildArtifacts: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            _commands: Commands<'w, 's>,
                        }
                        struct Input {
                            inner: BuildArtifactsInput,
                        }
                        struct Output {
                            inner: BuildArtifactsOutput,
                        }
                        enum Error {
                            Inner(HydrateChunkVisualsError),
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<Output, Error> {
                            let _ = main_access;
                            let output = run_build_artifacts_async(input.inner).map_err(Error::Inner)?;
                            Ok(Output {
                                inner: output,
                            })
                        }
                    ]
                },

                CommitArtifacts: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: CommitArtifactsMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: BuildArtifactsOutput,
                        }
                        struct Output {
                            inner: CommitArtifactsOutput,
                        }
                        enum Error {
                            Inner(HydrateChunkVisualsError),
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<Output, Error> {
                            let output = run_commit_artifacts_ecs(input.inner, main_access.inner).map_err(Error::Inner)?;
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
