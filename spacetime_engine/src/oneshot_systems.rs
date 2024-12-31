use std::collections::HashMap;
use bevy::{ecs::system::SystemId, prelude::*};
use crate::player::bundles::PlayerBundle;

#[derive(Resource)]
pub struct MainOneshotSystems(pub HashMap<String, SystemId>);

impl FromWorld for MainOneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut main_oneshot_systems: MainOneshotSystems = MainOneshotSystems(HashMap::new());

        main_oneshot_systems.0.insert(
            "spawn_main_camera".into(),
            world.register_system(spawn_main_camera_oneshot_system)
        );
        main_oneshot_systems.0.insert(
            "spawn_main_player".into(),
            world.register_system(spawn_main_player_oneshot_system)
        );

        main_oneshot_systems
    }
}

fn spawn_main_player_oneshot_system(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn spawn_main_camera_oneshot_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}