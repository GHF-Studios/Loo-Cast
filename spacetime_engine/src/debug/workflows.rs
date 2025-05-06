use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Debug",
    workflows: [
        SpawnDebugObjects {
            user_imports: {
                use bevy::prelude::*;

                use crate::{
                    chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent,
                };
                use crate::debug::components::{DebugObjectComponent, DebugObjectMovement};
            },
            user_items: {
                pub fn spawn_debug_object(
                    commands: &mut Commands,
                    position: Vec2,
                    rotation: f32,
                    scale: Vec2,
                    movement: DebugObjectMovement,
                ) {
                    commands.spawn((
                        ChunkActorComponent,
                        ChunkLoaderComponent::default(),
                        DebugObjectComponent { movement },
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
                Spawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| {
                            debug!("Spawning debug objects..");
                            let mut commands = main_access.commands;

                            spawn_debug_object(
                                &mut commands,
                                Vec2::new(350.0, 350.0),
                                0.0,
                                Vec2::ONE,
                                DebugObjectMovement::Circle {
                                    radius: 200.0,
                                    speed: 0.15,
                                },
                            );

                            spawn_debug_object(
                                &mut commands,
                                Vec2::new(-300.0, -400.0),
                                0.0,
                                Vec2::ONE,
                                DebugObjectMovement::Line {
                                    distance: 500.0,
                                    speed: 0.15,
                                },
                            );

                            spawn_debug_object(
                                &mut commands,
                                Vec2::new(-350.0, 400.0),
                                0.0,
                                Vec2::ONE,
                                DebugObjectMovement::Static,
                            );
                        }
                    ]
                }
            ]
        }
    ]
}
