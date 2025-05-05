use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
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
                Spawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| {
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
                    ]
                }
            ]
        }
    ]
}
