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

fn insert_physical_constants_resource_system(
    bevy_commands: &mut bevy::prelude::Commands,
    component_query: &ComponentQuery,
) {
    println!("Inserting Physical Constants Resource");
}