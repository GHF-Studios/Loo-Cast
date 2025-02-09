use std::collections::HashMap;
use bevy::render::renderer::{RenderAdapter, RenderDevice};
use bevy::{ecs::system::SystemId, prelude::*};
use crate::action::functions::request_action;
use crate::action::resources::ActionTypeModuleRegistry;
use crate::action::stage_io::ActionIO;
use crate::action::types::RawActionData;
use crate::camera::components::MainCamera;
use crate::config::statics::CONFIG;
use crate::debug::components::TestObjectMovement;
use crate::follower::components::{FollowerComponent, FollowerTargetComponent};
use crate::gpu::actions::setup_texture_generator;
use crate::player::bundles::PlayerBundle;
use crate::debug::functions::spawn_test_object;

#[derive(Resource)]
pub struct MainOneshotSystems(pub HashMap<String, SystemId>);

impl FromWorld for MainOneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut main_oneshot_systems: MainOneshotSystems = MainOneshotSystems(HashMap::new());

        main_oneshot_systems.0.insert(
            "initialize_action_type_modules".into(),
            world.register_system(initialize_action_type_modules_oneshot_system)
        );
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

fn initialize_action_type_modules_oneshot_system(
    mut action_type_module_registry: ResMut<ActionTypeModuleRegistry>
) {
    crate::chunk::actions::initialize_action_type_module(&mut action_type_module_registry);
    crate::gpu::actions::initialize_action_type_module(&mut action_type_module_registry);
}

fn test_action_framework_oneshot_system(world: &mut World) {
    use crate::gpu::actions::generate_texture;
    use crate::chunk::actions::spawn;
    use bevy::render::render_resource::PipelineCache;
    use bevy::ecs::system::SystemState;

    let mut system_state: SystemState<(
        Res<RenderDevice>,
        Res<RenderAdapter>,
    )> = SystemState::new(world);
    let (render_device, render_adapter) = system_state.get_mut(world);

    let render_device = render_device.clone();
    let render_adapter = render_adapter.clone();

    world.insert_resource(PipelineCache::new(
        render_device,
        render_adapter,
        true,
    ));


    if let Err(err) = request_action(
        world,
        "GPU",
        "SetupTextureGenerator",
        RawActionData::new(setup_texture_generator::Input(setup_texture_generator::SetupPipelineInput {
            shader_name: "example_shader", // TODO: Add real shader
            shader_path: "assets/shaders/example_compute_uv.wgsl".to_string(), // TODO: Add real shader
        })),
        Some(Box::new(|world, io| {
            io.consume_cast::<setup_texture_generator::Output>().0.unwrap_or_else(|err| { unreachable!("Failed to setup texture generator: {}", err) });
            debug!("Setup texture generator");

            if let Err(err) = request_action(
                world,
                "GPU",
                "GenerateTexture",
                RawActionData::new(generate_texture::Input(generate_texture::GenerateTextureInput {
                    shader_name: "example_shader".to_string(), // TODO: Add real shader
                    texture_size: CONFIG.get::<f32>("chunk/size") as usize
                })),
                Some(Box::new(|world, io| {
                    let output = io.consume_cast::<generate_texture::Output>().0.unwrap_or_else(|err| { unreachable!("Failed to generate texture: {}", err) });
                    debug!("Generated texture");
        
                    if let Err(err) = request_action(
                        world,
                        "Chunk",
                        "Spawn",
                        RawActionData::new(spawn::Input(spawn::SetupAndSpawnEntityInput {
                            chunk_coord: (0, 0),
                            chunk_owner: None,
                            metric_texture: output
                        })),
                        Some(Box::new(|_world, io| {
                            io.consume_cast::<spawn::Output>().0.unwrap_or_else(|err| { unreachable!("Failed to spawn chunk: {}", err) });
                            debug!("Spawned chunk");
                        }))
                    ) { 
                        debug!("Failed to spawn chunk: Failed request: {}", err) 
                    }
                }))
            ) { 
                debug!("Failed to generate texture: Failed request: {}", err) 
            }
        }))
    ) {
        debug!("Failed to setup texture generator: Failed request: {}", err)
    }
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