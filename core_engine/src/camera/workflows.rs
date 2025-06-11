use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Camera",
    workflows: [
        SpawnMainCamera {
            user_imports: {
                use bevy::prelude::{Commands, Res, ResMut, Camera2dBundle, Vec2};

                use crate::camera::components::MainCamera;
                use crate::config::statics::CONFIG;
                use crate::follower::components::{Follower, FollowerTarget};
            },
            user_items: {},
            stages: [
                SpawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                        struct State {
                            camera_entity: Entity,
                        }
                        struct Output {
                            spawned_camera_entity: Entity,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> State {
                            let mut commands = main_access.commands;

                            let camera_entity = commands.spawn((
                                Camera2dBundle::default(),
                                MainCamera,
                                Follower::new(
                                    "main_camera".to_string(),
                                    Vec2::ZERO,
                                    CONFIG.get::<f32>("camera/follow_smoothness"),
                                ),
                            )).id();

                            State {
                                camera_entity
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            let mut commands = main_access.commands;

                            if commands.get_entity(state.camera_entity).is_some() {
                                Outcome::Done(Output {
                                    spawned_camera_entity: state.camera_entity
                                })
                            } else {
                                Outcome::Wait(state)
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
