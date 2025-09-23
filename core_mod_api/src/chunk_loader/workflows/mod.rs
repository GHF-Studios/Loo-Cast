pub mod external;

use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "ChunkLoader",
    workflows: [
        CategorizeChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::chunk_loader::workflows::external::categorize_chunks::{
                    MainAccess as CategorizeStageMainAccess,
                    Output as CategorizeStageOutput,
                    run_ecs as categorize_stage_run_ecs
                };
                use crate::usf::scale::ScaleMeter1;
            },
            user_items: {
            },
            stages: [
                Categorize: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: CategorizeStageMainAccess<'w, 's, ScaleMeter1>,
                        }

                        struct Output {
                            inner: CategorizeStageOutput<ScaleMeter1>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Output {
                            let output = categorize_stage_run_ecs(main_access.inner);
                            Output { inner: output }
                        }
                    ]
                }
            ]
        }

        OnRemoveChunkLoader, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::chunk_loader::workflows::external::on_remove_chunk_loader::{
                    MainAccess as ExtractUnloadChunkInputsStageMainAccess,
                    Input as ExtractUnloadChunkInputsStageInput,
                    Output as ExtractUnloadChunkInputsStageOutput,
                    run_ecs as extract_unload_chunk_input_stage_run_ecs
                };
                use crate::usf::scale::ScaleMeter1;
            },
            user_items: {},
            stages: [
                ExtractUnloadChunkInputs: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMeter1>,
                        }
                        struct Input {
                            inner: ExtractUnloadChunkInputsStageInput<ScaleMeter1>,
                        }
                        struct Output {
                            inner: ExtractUnloadChunkInputsStageOutput<ScaleMeter1>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let output = extract_unload_chunk_input_stage_run_ecs(input.inner, main_access.inner);
                            Output { inner: output }
                        }
                    ]
                }
            ],
        }

        OnRemovedChunkLoader, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::chunk_loader::workflows::external::on_removed_chunk_loader::{
                    MainAccess as SendRemovedChunkLoaderEventStageMainAccess,
                    Input as SendRemovedChunkLoaderEventStageInput,
                    run_ecs as send_removed_chunk_loader_event_stage_run_ecs
                };
                use crate::usf::scale::ScaleMeter1;
            },
            user_items: {},
            stages: [
                SendRemovedChunkLoaderEvent: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMeter1>,
                        }
                        struct Input {
                            inner: SendRemovedChunkLoaderEventStageInput<ScaleMeter1>,
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
                use crate::chunk_loader::workflows::external::load_chunks::{
                    MainAccess as ValidateAndLoadAndWaitStageMainAccess,
                    Input as ValidateAndLoadAndWaitStageInput,
                    State as ValidateAndLoadAndWaitStageState,
                    setup_ecs_while as validate_and_load_and_wait_stage_setup_ecs_while,
                    run_ecs_while as validate_and_load_and_wait_stage_run_ecs_while,
                };
                use crate::usf::scale::ScaleMeter1;
            },
            user_items: {
            },
            stages: [
                ValidateAndLoadAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMeter1>,
                        }
                        struct Input {
                            inner: ValidateAndLoadAndWaitStageInput<ScaleMeter1>,
                        }
                        struct State {
                            inner: ValidateAndLoadAndWaitStageState<ScaleMeter1>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let state = validate_and_load_and_wait_stage_setup_ecs_while(input.inner, main_access.inner);
                            State { inner: state }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let outcome = validate_and_load_and_wait_stage_run_ecs_while(state.inner, main_access.inner);
                            outcome.map_wait(|s| State { inner: s })
                        }
                    ]
                }
            ]
        }

        UnloadChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use crate::chunk_loader::workflows::external::unload_chunks::{
                    MainAccess as UnloadAndWaitStageMainAccess,
                    Input as UnloadAndWaitStageInput,
                    State as UnloadAndWaitStageState,
                    setup_ecs_while as unload_and_wait_stage_setup_ecs_while,
                    run_ecs_while as unload_and_wait_stage_run_ecs_while,
                };
                use crate::usf::scale::ScaleMeter1;
            },
            user_items: {
            },
            stages: [
                UnloadAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner: UnloadAndWaitStageMainAccess<'w, 's, ScaleMeter1>,
                        }
                        struct Input {
                            inner: UnloadAndWaitStageInput<ScaleMeter1>,
                        }
                        struct State {
                            inner: UnloadAndWaitStageState<ScaleMeter1>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let state = unload_and_wait_stage_setup_ecs_while(input.inner, main_access.inner);
                            State { inner: state }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let outcome = unload_and_wait_stage_run_ecs_while(state.inner, main_access.inner);
                            outcome.map_wait(|s| State { inner: s })
                        }
                    ]
                }
            ]
        }
    ]
}
