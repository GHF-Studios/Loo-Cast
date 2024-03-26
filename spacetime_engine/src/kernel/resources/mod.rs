use spacetime_engine_derive::define_resources_module;

define_resources_module! {
    Test {
        module_path: crate::kernel::resources,
        resources: [
            struct TestCommands,
        ]
    }
}