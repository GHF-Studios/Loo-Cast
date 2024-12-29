use std::collections::HashMap;
use bevy::{ecs::system::SystemId, prelude::*};
use crate::player::bundles::PlayerBundle;

#[derive(Resource)]
pub struct MainSystems(pub HashMap<String, SystemId>);
impl FromWorld for MainSystems {
    fn from_world(world: &mut World) -> Self {
        let mut main_systems: MainSystems = MainSystems(HashMap::new());

        main_systems.0.insert(
            "spawn_main_camera".into(),
            world.register_system(spawn_main_camera_system)
        );
        main_systems.0.insert(
            "spawn_player".into(),
            world.register_system(spawn_player_system)
        );

        main_systems
    }
}

fn spawn_player_system(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn spawn_main_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}