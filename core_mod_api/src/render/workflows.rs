use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Render",
    workflows: [
        SpawnCameras, timeout_secs: 5.0, timeout_mode: RealTime {
            user_imports: {
                use bevy::prelude::{Commands, Res, ResMut, Camera2d, Vec2, Name, Camera};
                use bevy::math::FloatOrd;
                use bevy::render::view::RenderLayers;
                use bevy_egui::EguiRenderOutput;
                use bevy::render::render_resource::{
                    Buffer, TextureView, TextureDescriptor, Extent3d,
                    TextureDimension, TextureFormat, TextureUsages,
                };
                use bevy::render::camera::{RenderTarget, ImageRenderTarget};
                use bevy::window::WindowRef;
                use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;

                use crate::chunk::components::ChunkActor;
                use crate::config::statics::CONFIG;
                use crate::follower::components::{Follower, FollowerTarget};
                use crate::render::{
                    components::MainCamera,
                    functions::get_reserved_camera_entities,
                    resources::GameViewRenderTarget
                };
            },
            user_items: {},
            stages: [
                SpawnAndWait: EcsWhile, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            game_view_render_target: Res<'w, GameViewRenderTarget>,
                        }
                        struct State {
                            main_camera_entity: Entity,
                            ui_camera_entity: Entity,
                            egui_camera_entity: Entity,
                        }
                        struct Output {
                            spawned_main_camera_entity: Entity,
                            spawned_ui_camera_entity: Entity,
                            spawned_egui_camera_entity: Entity,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> State {
                            let mut commands = main_access.commands;
                            let game_view_render_target = main_access.game_view_render_target;

                            let (
                                egui_camera_entity,
                                ui_camera_entity,
                                main_camera_entity,
                            ) = get_reserved_camera_entities();

                            commands.entity(egui_camera_entity).insert((
                                Name::new("egui_camera"),
                                Camera2d,
                                Camera {
                                    order: 2,
                                    target: RenderTarget::Window(WindowRef::Primary),
                                    ..Default::default()
                                },
                                PrimaryEguiContext,
                                EguiRenderOutput::default(),
                            ));
                            commands.entity(ui_camera_entity).insert((
                                Name::new("ui_camera"),
                                Camera2d,
                                Camera {
                                    order: 1,
                                    target: RenderTarget::Image(ImageRenderTarget {
                                        handle: game_view_render_target.handle.clone(),
                                        scale_factor: FloatOrd(1.0),
                                    }),
                                    ..Default::default()
                                },
                                RenderLayers::layer(1),
                            ));
                            commands.entity(main_camera_entity).insert((
                                Name::new("main_camera"),
                                Camera2d,
                                Camera {
                                    order: 0,
                                    target: RenderTarget::Image(ImageRenderTarget {
                                        handle: game_view_render_target.handle.clone(),
                                        scale_factor: FloatOrd(1.0),
                                    }),
                                    ..Default::default()
                                },
                                MainCamera,
                                RenderLayers::default(),
                                ChunkActor::default(),
                                Follower::new(
                                    "main_camera".to_string(),
                                    Vec2::ZERO,
                                    CONFIG().get::<f32>("camera/follow_smoothness"),
                                ),
                                FollowerTarget {
                                    id: "main_camera_proxy".to_string(),
                                },
                            ));

                            State {
                                main_camera_entity,
                                ui_camera_entity,
                                egui_camera_entity,
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            let mut commands = main_access.commands;

                            if commands.get_entity(state.main_camera_entity).is_ok()
                                && commands.get_entity(state.ui_camera_entity).is_ok()
                                && commands.get_entity(state.egui_camera_entity).is_ok()
                            {
                                Outcome::Done(Output {
                                    spawned_main_camera_entity: state.main_camera_entity,
                                    spawned_ui_camera_entity: state.ui_camera_entity,
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
