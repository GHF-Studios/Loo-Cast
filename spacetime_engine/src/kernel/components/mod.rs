use spacetime_engine_macro::define_components_module;

define_components_module! {
    Test {
        module_path: crate::kernel::components,
        components: [
            struct Point {
                x: i32,
                y: i32,
            },
            enum Gizmo {
                Line {
                    start_point: Point,
                    end_point: Point,
                },
                Circle {
                    center: Point,
                    radius: i32,
                },
            },
        ]
    }
}