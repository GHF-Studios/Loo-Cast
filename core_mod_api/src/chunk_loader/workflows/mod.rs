pub mod external;

use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "ChunkLoader",
    workflows: [
        CategorizeChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;

                use crate::chunk_loader::workflows::external::categorize_chunks::{
                    MainAccess as CategorizeStageMainAccess,
                    Output as CategorizeStageOutput,
                    run_ecs as categorize_stage_run_ecs
                };
                use crate::usf::scale::*;
            },
            user_items: {
            },
            stages: [
                Categorize: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: CategorizeStageMainAccess<'w, 's>,
                        }

                        struct Output {
                            inner: CategorizeStageOutput,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Output {
                            let output = categorize_stage_run_ecs(main_access.inner);

                            Output {
                                inner: output,
                            }
                        }
                    ]
                }
            ]
        }

        OnRemoveChunkLoader, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::on_remove_chunk_loader::{
                    MainAccess as ExtractUnloadChunkInputsStageMainAccess,
                    Input as ExtractUnloadChunkInputsStageInput,
                    Output as ExtractUnloadChunkInputsStageOutput,
                    run_ecs as extract_unload_chunk_input_stage_run_ecs
                };
                use crate::usf::scale::*;
            },
            user_items: {},
            stages: [
                ExtractUnloadChunkInputs: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: ExtractUnloadChunkInputsStageMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: ExtractUnloadChunkInputsStageInput,
                        }
                        struct Output {
                            inner: ExtractUnloadChunkInputsStageOutput,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let output = extract_unload_chunk_input_stage_run_ecs(input.inner, main_access.inner);

                            Output {
                                inner: output,
                            }
                        }
                    ]
                }
            ],
        }

        OnRemovedChunkLoader, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::on_removed_chunk_loader::{
                    MainAccess as SendRemovedChunkLoaderEventStageMainAccess,
                    Input as SendRemovedChunkLoaderEventStageInput,
                    run_ecs as send_removed_chunk_loader_event_stage_run_ecs
                };
                use crate::usf::scale::*;
            },
            user_items: {},
            stages: [
                SendRemovedChunkLoaderEvent: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: SendRemovedChunkLoaderEventStageMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: SendRemovedChunkLoaderEventStageInput,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| {
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner, main_access.inner);
                        }
                    ]
                }
            ],
        }

        LoadChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::load_chunks::{
                    MainAccess as ValidateAndLoadAndWaitStageMainAccess,
                    Input as ValidateAndLoadAndWaitStageInput,
                    State as ValidateAndLoadAndWaitStageState,
                    setup_ecs_while as validate_and_load_and_wait_stage_setup_ecs_while,
                    run_ecs_while as validate_and_load_and_wait_stage_run_ecs_while,
                };
                use crate::usf::scale::*;
            },
            user_items: {
            },
            stages: [
                ValidateAndLoadAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: ValidateAndLoadAndWaitStageMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: ValidateAndLoadAndWaitStageInput,
                        }
                        struct State {
                            inner: Option<ValidateAndLoadAndWaitStageState>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let state = validate_and_load_and_wait_stage_setup_ecs_while(input.inner, main_access.inner);
                            
                            State {
                                inner: Some(state),
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let outcome = if let Some(state) = state.inner {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner))
                            } else { None };

                            if outcome.is_none() {
                                return Done(());
                            }

                            let outcome = match outcome {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };

                            Wait(State {
                                inner: outcome,
                            })
                        }
                    ]
                }
            ]
        }

        UnloadChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::unload_chunks::{
                    MainAccess as UnloadAndWaitStageMainAccess,
                    Input as UnloadAndWaitStageInput,
                    State as UnloadAndWaitStageState,
                    setup_ecs_while as unload_and_wait_stage_setup_ecs_while,
                    run_ecs_while as unload_and_wait_stage_run_ecs_while,
                };
                use crate::usf::scale::*;
            },
            user_items: {
            },
            stages: [
                UnloadAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: UnloadAndWaitStageMainAccess<'w, 's>,
                        }
                        struct Input {
                            inner: UnloadAndWaitStageInput,
                        }
                        struct State {
                            inner: Option<UnloadAndWaitStageState>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let state = unload_and_wait_stage_setup_ecs_while(input.inner, main_access.inner);
                            
                            State {
                                inner: Some(state),
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let outcome = if let Some(state) = state.inner {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner))
                            } else { None };

                            if outcome.is_none() {
                                return Done(());
                            }

                            let outcome = match outcome {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };

                            Wait(State {
                                inner: outcome,
                            })
                        }
                    ]
                }
            ]
        }
    ]
}
