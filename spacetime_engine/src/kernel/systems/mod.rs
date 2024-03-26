use spacetime_engine_derive::define_systems_module;

define_systems_module! {
    Test {
        module_path: crate::kernel::systems,
        systems: [
            HelloWorld {
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