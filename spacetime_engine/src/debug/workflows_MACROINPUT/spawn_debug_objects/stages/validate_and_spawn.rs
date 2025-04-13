crate::workflow_stage_util!("ValidateAndSpawn");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        commands: Commands<'w, 's>,
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(main_access: MainAccess) {
        let mut commands = main_access.commands;
        
        spawn_test_object(
            &mut commands,
            Vec2::new(350.0, 350.0),
            0.0,
            Vec2::ONE,
            TestObjectMovement::Circle {
                radius: 200.0,
                speed: 0.15,
            },
        );

        spawn_test_object(
            &mut commands,
            Vec2::new(-300.0, -400.0),
            0.0,
            Vec2::ONE,
            TestObjectMovement::Line {
                distance: 500.0,
                speed: 0.15,
            },
        );

        spawn_test_object(
            &mut commands,
            Vec2::new(-350.0, 400.0),
            0.0,
            Vec2::ONE,
            TestObjectMovement::Static,
        );
    }
}
