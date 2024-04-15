use spacetime_engine_macro::define_systems_module;

define_systems_module! {
    Test {
        module_path: crate::kernel::systems,
        systems: [
            InsertPhysicalConstantsResource {
                Input {
                    BevyCommands {
                        
                    }
                    ComponentQuery {

                    },
                    ResourceQuery
                },
                Schedule {
                    OnEnter(AppState::MainMenu),
                    OnUpdate,
                    OnExit(AppState::MainMenu),
                },
                Code || -> () {
                    println!("Hello World!");
                }
            }
        ]
    }
}