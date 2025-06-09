use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Debug",
    workflows: [
        SpawnDebugObjects {
            user_imports: {
                use bevy::prelude::{Commands, Entity, SpriteBundle, Sprite, Color, Rect, Transform, Quat, Vec2, Res, ResMut};

                use crate::{
                    chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent,
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
                    commands.spawn((
                        ChunkActorComponent,
                        ChunkLoaderComponent::new(chunk_loader_id),
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
                SpawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>
                        }
                        struct State {
                            circle_entity: Entity,
                            line_entity: Entity,
                            static_entity: Entity,
                            is_circle_entity_spawned: bool,
                            is_line_entity_spawned: bool,
                            is_static_entity_spawned: bool,
                        }
                        struct Output {
                            circle_entity: Entity,
                            line_entity: Entity,
                            static_entity: Entity,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> State {
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

                            State {
                                circle_entity,
                                line_entity,
                                static_entity,
                                is_circle_entity_spawned: false,
                                is_line_entity_spawned: false,
                                is_static_entity_spawned: false,
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, Output> {
                            let mut commands = main_access.commands;

                            let mut updated_state = State {
                                circle_entity: state.circle_entity,
                                line_entity: state.line_entity,
                                static_entity: state.static_entity,
                                is_circle_entity_spawned: state.is_circle_entity_spawned,
                                is_line_entity_spawned: state.is_line_entity_spawned,
                                is_static_entity_spawned: state.is_static_entity_spawned,
                            };

                            if commands.get_entity(updated_state.circle_entity).is_some() {
                                updated_state.is_circle_entity_spawned = true;
                            }
                            if commands.get_entity(updated_state.line_entity).is_some() {
                                updated_state.is_line_entity_spawned = true;
                            }
                            if commands.get_entity(updated_state.static_entity).is_some() {
                                updated_state.is_static_entity_spawned = true;
                            }

                            if state.is_circle_entity_spawned && state.is_line_entity_spawned && state.is_static_entity_spawned {
                                Outcome::Done(Output {
                                    circle_entity: state.circle_entity,
                                    line_entity: state.line_entity,
                                    static_entity: state.static_entity,
                                })
                            } else {
                                Outcome::Wait(updated_state)
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
