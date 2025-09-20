use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Camera",
    workflows: [
        SpawnMainCameras, timeout_secs: 5.0, timeout_mode: RealTime {
            user_imports: {
                use bevy::prelude::{Commands, Res, ResMut, Camera2d, Vec2, Name, Camera};
                use bevy::render::view::RenderLayers;
                use bevy_egui::EguiRenderOutput;
                use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
                use bevy::render::render_resource::{
                    Buffer, TextureView, TextureDescriptor, Extent3d,
                    TextureDimension, TextureFormat, TextureUsages,
                };
                use bevy::render::camera::RenderTarget;
                use bevy::window::WindowRef;

                use crate::camera::components::MainCamera;
                use crate::config::statics::CONFIG;
                use crate::follower::components::{Follower, FollowerTarget};
            },
            user_items: {},
            stages: [
                SpawnAndWait: EcsWhile, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
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

                            let egui_camera_entity = commands.spawn((
                                Camera2d,
                                Camera {
                                    order: 1,
                                    ..Default::default()
                                },
                                Name::new("egui_camera_entity"),
                                PrimaryEguiContext,
                                EguiRenderOutput::default(),
                                RenderLayers::none(),
                            )).id();
                            let main_camera_entity = commands.spawn((
                                Camera2d,
                                Camera {
                                    target: RenderTarget::Window(WindowRef::Primary),
                                    ..Default::default()
                                },
                                Name::new("main_camera_entity"),
                                MainCamera,
                                Follower::new(
                                    "main_camera".to_string(),
                                    Vec2::ZERO,
                                    CONFIG().get::<f32>("camera/follow_smoothness"),
                                ),
                            )).id();

                            State {
                                main_camera_entity,
                                egui_camera_entity
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            let mut commands = main_access.commands;

                            if commands.get_entity(state.main_camera_entity).is_ok()
                                && commands.get_entity(state.egui_camera_entity).is_ok()
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
