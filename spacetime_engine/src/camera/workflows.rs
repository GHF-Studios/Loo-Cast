use spacetime_engine_macros::define_workflow_mod;

define_workflow_mod! {
    name: "Camera",
    workflows: [
        SpawnMainCamera {
            user_imports: {
                use bevy::prelude::*;

                use crate::camera::components::MainCamera;
                use crate::config::statics::CONFIG;
                use crate::follower::components::{FollowerComponent, FollowerTargetComponent};
            },
            user_items: {},
            stages: [
                ValidateAndSpawn: Ecs {
                    core_types: [],
                    core_functions: [
                        fn RunEcs |world| {
                            world.spawn((
                                Camera2dBundle::default(),
                                MainCamera,
                                FollowerComponent::new(
                                    "main_camera".to_string(),
                                    Vec2::ZERO,
                                    CONFIG.get::<f32>("camera/follow_smoothness"),
                                ),
                            ));
                        }
                    ]
                }
            ]
        }
    ]
}
