use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Camera",
    workflows: [
        SpawnMainCamera {
            user_imports: {
                use bevy::prelude::{Commands, Query, Entity, Camera2dBundle, Res, ResMut, Vec2};

                use crate::camera::components::MainCamera;
                use crate::config::statics::CONFIG;
                use crate::follower::components::{FollowerComponent, FollowerTargetComponent};
            },
            user_items: {},
            stages: [
                Spawn: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            entity_query: Query<'w, 's, Entity>
                        }
                        struct State {
                            camera_entity: Entity
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |main_access| -> State {
                            bevy::prelude::debug!("Spawning main camera..");
                            let mut commands = main_access.commands;

                            let entity_commands = commands.spawn((
                                Camera2dBundle::default(),
                                MainCamera,
                                FollowerComponent::new(
                                    "main_camera".to_string(),
                                    Vec2::ZERO,
                                    CONFIG.get::<f32>("camera/follow_smoothness"),
                                ),
                            ));
                            let camera_entity = entity_commands.id();

                            State { camera_entity }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let entity_query = main_access.entity_query;
                            
                            let camera_entity = state.camera_entity;

                            if entity_query.get(camera_entity).is_ok() {
                                Done(())
                            } else {
                                Wait(state)
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
