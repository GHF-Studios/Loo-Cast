define_objects_module! {
    Test {
        module_path: crate::kernel::objects,
        objects: [
            Player {
                Components {
                    Test::Position,
                    Test::Health,
                }
            }
        ]
    }
}