use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Debug",
    workflows: [
        SpawnDebugUI {
            user_imports: {
                use iyes_perf_ui::{
                    entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
                    prelude::{PerfUiEntryEntityCount, PerfUiRoot},
                };
            },
            user_items: {},
            stages: [
                ValidateAndSpawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| {
                            let mut commands = main_access.commands;
                            
                            commands.spawn((
                                PerfUiRoot::default(),
                                PerfUiFramerateEntries::default(),
                                PerfUiSystemEntries::default(),
                                PerfUiEntryEntityCount::default(),
                            ));
                        }
                    ]
                }
            ]
        }
        
        SpawnDebugObjects {
            user_imports: {
                use bevy::prelude::*;

                use crate::{
                    chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent,
                };
                use crate::debug::components::{TestObjectComponent, TestObjectMovement};
            },
            user_items: {
                pub fn spawn_test_object(
                    commands: &mut Commands,
                    position: Vec2,
                    rotation: f32,
                    scale: Vec2,
                    movement: TestObjectMovement,
                ) {
                    commands.spawn((
                        ChunkActorComponent,
                        ChunkLoaderComponent::default(),
                        TestObjectComponent { movement },
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.0, 0.0, 1.0),
                                rect: Some(Rect::new(-16.0, -16.0, 16.0, 16.0)),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: position.extend(0.0),
                                rotation: Quat::from_rotation_z(rotation),
                                scale: scale.extend(1.0),
                            },
                            ..Default::default()
                        },
                    ));
                }
            },
            stages: [
                ValidateAndSpawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| {
                            let mut commands = main_access.commands;
                            
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
                    ]
                }
            ]
        }
    ]
}
