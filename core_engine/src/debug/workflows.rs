use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Debug",
    workflows: [
        SpawnDebugObjects {
            user_imports: {
                use bevy::prelude::{Commands, Entity, SpriteBundle, Sprite, Color, Rect, Transform, Quat, Vec2, Res, ResMut};

                use crate::{
                    chunk_actor::components::ChunkActor, chunk_loader::components::ChunkLoader,
                };
                use crate::debug::components::{DebugObjectComponent, DebugObjectMovement};
            },
            user_items: {
                pub fn spawn_debug_object(
                    commands: &mut Commands,
                    chunk_loader_id: String,
                    position: Vec2,
                    rotation: f32,
                    scale: Vec2,
                    movement: DebugObjectMovement,
                ) -> Entity {
                    let chunk_loader = ChunkLoader::new(chunk_loader_id);

                    commands.entity(chunk_loader.chunk_owner_id().entity()).insert((
                        ChunkActor,
                        chunk_loader,
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
                    )).id()
                }
            },
            stages: [
                Spawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                        struct Output {
                            circle_entity: Entity,
                            line_entity: Entity,
                            static_entity: Entity,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Output {
                            let mut commands = main_access.commands;

                            let circle_entity = spawn_debug_object(
                                &mut commands,
                                "circle_entity_chunk_loader".to_string(),
                                Vec2::new(350.0, 350.0),
                                0.0,
                                Vec2::ONE,
                                DebugObjectMovement::Circle {
                                    radius: 200.0,
                                    speed: 0.15,
                                },
                            );

                            let line_entity = spawn_debug_object(
                                &mut commands,
                                "line_entity_chunk_loader".to_string(),
                                Vec2::new(-300.0, -400.0),
                                0.0,
                                Vec2::ONE,
                                DebugObjectMovement::Line {
                                    distance: 500.0,
                                    speed: 0.15,
                                },
                            );

                            let static_entity = spawn_debug_object(
                                &mut commands,
                                "static_entity_chunk_loader".to_string(),
                                Vec2::new(-350.0, 400.0),
                                0.0,
                                Vec2::ONE,
                                DebugObjectMovement::Static,
                            );

                            Output {
                                circle_entity,
                                line_entity,
                                static_entity,
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
