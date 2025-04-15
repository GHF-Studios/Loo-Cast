
pub const NAME: &str = stringify!("ValidateAndSpawn");

pub mod core_types {
    use super::super::super::imports:: * ;
    use bevy::prelude:: * ;
    #[derive(bevy::ecs::system::SystemParam)]
    pub struct MainAccess<'w,'s>{
        pub commands:Commands<'w,'s>
    }
}

pub mod core_functions {
    use super::super::super::imports:: * ;
    use super::core_types:: * ;
    pub fn run_ecs(_input:Option<Box<dyn std::any::Any+Send+Sync>> ,main_access:Box<dyn std::any::Any+Send+Sync>) -> Option<Box<dyn std::any::Any+Send+Sync>>{
        let main_access = main_access.downcast:: <MainAccess>().unwrap();
        run_ecs_inner(*main_access);
        None
    }
    fn run_ecs_inner(main_access:MainAccess){
        let mut commands = main_access.commands;
        commands.spawn((PlayerBundle::default(),FollowerTargetComponent {
            id:"main_camera".to_string(),
        },));
    }
}