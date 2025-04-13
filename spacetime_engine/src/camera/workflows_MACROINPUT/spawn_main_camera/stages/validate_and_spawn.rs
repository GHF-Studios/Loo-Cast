crate::workflow_stage_util!("ValidateAndSpawn");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        pub commands: Commands<'w, 's>,
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();

    pub fn run_ecs_inner(mut main_access: MainAccess) {
        main_access.commands.spawn((
            Camera2dBundle::default(),
            MainCamera,
            FollowerComponent::new(
                "main_camera".to_string(),
                Vec2::ZERO,
                CONFIG.get::<f32>("camera/follow_smoothness"),
            ),
        ));
    }
    crate::workflow_stage_core_function_util!(run_ecs);
}
