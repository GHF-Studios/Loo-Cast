use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Render",
    workflows: [
        SpawnCameras, timeout_secs: 5.0, timeout_mode: RealTime {
            user_imports: {
                use crate::bevy::prelude::{
                    Commands, Res, ResMut, Camera2d, Camera3d, Vec3, Name, Camera, Projection, PerspectiveProjection, Quat, Transform, DirectionalLight,
                    EulerRot
                };
                use bevy_egui::{EguiGlobalSettings, EguiRenderOutput};
                use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;

                use crate::usf::chunk::components::ChunkActor;
                use crate::config::statics::CONFIG;
                use crate::follower::components::{Follower, FollowerTarget};
                use crate::render::{
                    camera_contract,
                    components::{EguiCamera, MainCamera},
                    functions::get_reserved_camera_entities,
                    resources::GameViewRenderTarget
                };
                use crate::usf::scale::Scale;
            },
            user_items: {},
            stages: [
                SpawnAndWait: EcsWhile, run_if_paused: true, run_after_startup_finished: false {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            game_view_render_target: Res<'w, GameViewRenderTarget>,
                            egui_global_settings: ResMut<'w, EguiGlobalSettings>,
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
                            let mut egui_global_settings = main_access.egui_global_settings;
                            egui_global_settings.auto_create_primary_context = false;

                            let (
                                egui_camera_entity,
                                ui_camera_entity,
                                main_camera_entity,
                            ) = get_reserved_camera_entities();

                            commands.entity(egui_camera_entity).insert((
                                Name::new("egui_camera"),
                                Camera2d,
                                camera_contract::egui_camera_component(),
                                camera_contract::primary_window_render_target(),
                                PrimaryEguiContext,
                                EguiRenderOutput::default(),
                                EguiCamera,
                            ));
                            commands.entity(ui_camera_entity).insert((
                                Name::new("ui_camera"),
                                Camera2d,
                                camera_contract::ui_camera_component(),
                                camera_contract::game_view_render_target(&game_view_render_target.handle),
                                camera_contract::ui_camera_render_layers(),
                            ));
                            commands.entity(main_camera_entity).insert((
                                Name::new("main_camera"),
                                Camera3d::default(),
                                camera_contract::main_camera_component(),
                                Projection::Perspective(PerspectiveProjection {
                                    fov: CONFIG().get::<f32>("camera/default_fov_degrees").to_radians(),
                                    near: 0.1,
                                    far: Scale::CANONICAL_CAMERA_FAR,
                                    ..Default::default()
                                }),
                                Transform {
                                    translation: Vec3::new(0.0, 0.0, Scale::CANONICAL_CAMERA_Z),
                                    rotation: Quat::IDENTITY,
                                    ..Default::default()
                                },
                                camera_contract::game_view_render_target(&game_view_render_target.handle),
                                MainCamera,
                                camera_contract::main_camera_render_layers(),
                                ChunkActor::default(),
                                Follower::new(
                                    "main_camera".to_string(),
                                    Vec3::ZERO,
                                    CONFIG().get::<f32>("camera/follow_smoothness"),
                                ),
                                FollowerTarget {
                                    id: "main_camera_proxy".to_string(),
                                },
                            ));
                            commands.spawn((
                                Name::new("phenomenon_model_directional_light"),
                                DirectionalLight::default(),
                                Transform {
                                    translation: Vec3::new(0.0, 0.0, Scale::CANONICAL_CAMERA_Z - 2_000.0),
                                    rotation: Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0),
                                    ..Default::default()
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
