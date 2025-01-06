use std::collections::HashMap;
use bevy::{ecs::system::SystemId, prelude::*};
use crate::camera::components::MainCameraFollowComponent;
use crate::debug::components::TestObjectMovement;
use crate::player::bundles::PlayerBundle;
use crate::debug::functions::spawn_test_object;

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
        main_oneshot_systems.0.insert(
            "spawn_main_test_objects".into(),
            world.register_system(spawn_main_test_objects_oneshot_system)
        );

        main_oneshot_systems
    }
}

fn spawn_main_player_oneshot_system(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn spawn_main_camera_oneshot_system(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCameraFollowComponent {
            target: None,
            speed: 5.0, // Adjust the speed for snappiness
        },
    ));
}

fn spawn_main_test_objects_oneshot_system(mut commands: Commands) {
    spawn_test_object(
        &mut commands,
        Vec2::new(350.0, 350.0),
        0.0,
        Vec2::ONE,
        TestObjectMovement::Circle {
            radius: 200.0,
            speed: 1.0,
        },
    );

    spawn_test_object(
        &mut commands,
        Vec2::new(-300.0, -400.0),
        0.0,
        Vec2::ONE,
        TestObjectMovement::Line {
            distance: 500.0,
            speed: 1.5,
        },
    );

    spawn_test_object(
        &mut commands,
        Vec2::new(-350.0, 300.0),
        0.0,
        Vec2::ONE,
        TestObjectMovement::Static,
    );
}