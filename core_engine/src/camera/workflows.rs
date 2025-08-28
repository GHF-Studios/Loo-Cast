use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Camera",
    workflows: [
        SpawnMainCameras {
            user_imports: {
                use bevy::prelude::{Commands, Res, ResMut, Camera2d, Vec2, Name, Camera};
                use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;

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
                            main_camera_entity: Entity,
                            egui_camera_entity: Entity,
                        }
                        struct Output {
                            spawned_main_camera_entity: Entity,
                            spawned_egui_camera_entity: Entity,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> State {
                            let mut commands = main_access.commands;

                            let main_camera_entity = commands.spawn((
                                Camera2d,
                                Name::new("main_camera_entity"),
                                MainCamera,
                                Follower::new(
                                    "main_camera".to_string(),
                                    Vec2::ZERO,
                                    CONFIG.get::<f32>("camera/follow_smoothness"),
                                ),
                            )).id();
                            let egui_camera_entity = commands.spawn((
                                Camera2d,
                                Name::new("egui_camera_entity"),
                                PrimaryEguiContext,
                                RenderLayers::none(),
                                Camera {
                                    order: 1,
                                    ..Default::default()
                                },
                            )).id();

                            State {
                                main_camera_entity,
                                egui_camera_entity
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            let mut commands = main_access.commands;

                            if commands.get_entity(state.main_camera_entity).is_some()
                                && commands.get_entity(state.egui_camera_entity).is_some()
                            {
                                Outcome::Done(Output {
                                    spawned_main_camera_entity: state.main_camera_entity,
                                    spawned_egui_camera_entity: state.egui_camera_entity,
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
