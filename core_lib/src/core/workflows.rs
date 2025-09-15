use core_lib_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Core",
    workflows: [
        FinishStartup, timeout_secs: 1.0, timeout_mode: RealTime {
            user_imports: {
                use bevy::prelude::Commands;

                use crate::core::resources::StartupFinished;
            },
            user_items: {},
            stages: [
                InsertResource: Ecs, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| {
                            let mut commands = main_access.commands;

                            commands.insert_resource(StartupFinished);
                        }
                    ]
                }
            ]
        }
    ]
}
