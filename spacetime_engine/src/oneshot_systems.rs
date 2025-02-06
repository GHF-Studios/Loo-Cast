use std::collections::HashMap;
use bevy::{ecs::system::SystemId, prelude::*};
use crate::action::functions::request_action;
use crate::action::stage_io::ActionIO;
use crate::camera::components::MainCamera;
use crate::config::statics::CONFIG;
use crate::debug::components::TestObjectMovement;
use crate::follower::components::{FollowerComponent, FollowerTargetComponent};
use crate::player::bundles::PlayerBundle;
use crate::debug::functions::spawn_test_object;

#[derive(Resource)]
pub struct MainOneshotSystems(pub HashMap<String, SystemId>);

impl FromWorld for MainOneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut main_oneshot_systems: MainOneshotSystems = MainOneshotSystems(HashMap::new());

        main_oneshot_systems.0.insert(
            "test_action_framework".into(),
            world.register_system(test_action_framework_oneshot_system)
        );
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

fn test_action_framework_oneshot_system(world: &mut World) {
    use crate::gpu::actions::generate_texture;
    use crate::chunk::actions::spawn;

    request_action(
        world,
        "GPU",
        "GenerateTexture",
        ActionIO::new(Box::new(generate_texture::Input(generate_texture::DoSomethingInput {
            texture_size: CONFIG.get::<f32>("chunk/size") as usize
        }))),
        Some(Box::new(|world, io| {
            debug!("Generated Metric Texture");
            let output: Handle<Image> = *io.consume_cast();

            request_action(
                world,
                "Chunk",
                "Spawn",
                ActionIO::new(Box::new(spawn::Input(spawn::GenerateMetricMapsInput {
                    chunk_coord: (0, 0),
                    chunk_owner: None,
                    metric_texture: output
                }))),
                Some(Box::new(|world, io| {
                    debug!("Spawned Chunk");
                    let _output = io.consume();
                }))
            );
        }))
    );
}

fn spawn_main_player_oneshot_system(mut commands: Commands) {
    commands.spawn((
        PlayerBundle::default(),
        FollowerTargetComponent {
            id: "player_camera".to_string(), 
        }
    ));
}

fn spawn_main_camera_oneshot_system(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        FollowerComponent::new(
            "player_camera".to_string(), 
            Vec2::ZERO, 
            CONFIG.get::<f32>("camera/follow_smoothness")
        )
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