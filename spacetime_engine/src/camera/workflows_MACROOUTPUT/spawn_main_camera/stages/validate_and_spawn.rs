pub const NAME: &str = stringify!("ValidateAndSpawn");

pub mod core_types {
    use bevy::ecs::system::SystemParam;

    use super::super::super::imports::*;
    use super::super::super::user_items::*;
    use super::core_functions::*;

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        pub commands: Commands<'w, 's>,
    }
}

pub mod core_functions {
    use bevy::ecs::system::SystemParam;

    use super::super::super::imports::*;
    use super::super::super::user_items::*;
    use super::core_types::*;

    pub fn run_ecs_inner(main_access: MainAccess) {
        let mut commands = main_access.commands;

        commands.spawn((
            Camera2dBundle::default(),
            MainCamera,
            FollowerComponent::new(
                "main_camera".to_string(),
                Vec2::ZERO,
                CONFIG.get::<f32>("camera/follow_smoothness"),
            ),
        ));
    }
    pub fn run_ecs(
        _input: Option<Box<dyn std::any::Any + Send + Sync>>,
        main_access: Box<dyn std::any::Any + Send + Sync>,
    ) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        let main_access = main_access.downcast::<MainAccess>().unwrap();
        run_ecs_inner(*main_access);
        None
    }
}
