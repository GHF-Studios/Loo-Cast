use std::any::TypeId;
use bevy::prelude::*;

use crate::{action::{resources::ActionTargetTypeRegistry, stage_io::*, structs::*}, config::statics::CONFIG};

use super::{components::ChunkComponent, functions::chunk_pos_to_world};

pub mod spawn {
    use bevy::prelude::*;

    pub struct GenerateMetricMapsInput {
        pub chunk_coord: (i32, i32),
        pub chunk_owner: Option<Entity>,
    }

    pub struct SetupAndSpawnEntityInput {
        pub chunk_coord: (i32, i32),
        pub chunk_owner: Option<Entity>,
        pub metric_texture: Handle<Image>,
    }

    pub struct Output(pub Result<(), String>);
}

pub mod despawn {
    use bevy::prelude::*;

    pub struct FindAndDespawnEntityInput {
        pub chunk_coord: (i32, i32),
    }

    pub struct Output(pub Result<(), String>);
}

fn register(
    action_target_type_registry: &mut ResMut<ActionTargetTypeRegistry>,
) {
    action_target_type_registry.register::<ChunkComponent>(
        ActionTargetType {
            name: "Chunk".to_owned(),
            type_id: TypeId::of::<ChunkComponent>(),
            ecs_action_types: vec![
                ActionType {
                    name: "Spawn".to_owned(),
                    stages: vec![
                        ActionStage::Async(ActionStageAsync {
                            name: "GenerateMetricMaps".to_owned(),
                            function: Box::pin(|io: ActionStageIO<InputState>| Box::pin(async move {
                                let (input, io) = io.get_input::<spawn::GenerateMetricMapsInput>();
                                let chunk_coord = input.chunk_coord;
                                let chunk_owner = input.chunk_owner;

                                // Simulate an async compute shader call
                                let metric_texture = async_compute_metric_texture(chunk_coord).await;

                                io.set_output(spawn::SetupAndSpawnEntityInput {
                                    chunk_coord,
                                    chunk_owner,
                                    metric_texture,
                                })
                            })),
                        }),
                        ActionStage::Ecs(ActionStageEcs {
                            name: "SetupAndSpawnEntity".to_owned(),
                            function: Box::new(|io: ActionStageIO<InputState>, world: &mut World| -> ActionStageIO<OutputState> {
                                let (input, io) = io.get_input::<spawn::SetupAndSpawnEntityInput>();
                                let chunk_coord = input.chunk_coord;
                                let chunk_owner = input.chunk_owner;
                                let metric_texture = input.metric_texture;

                                let half_chunk_size = CONFIG.get::<f32>("chunk/size") / 2.0;
                                let default_chunk_z = CONFIG.get::<f32>("chunk/default_z");

                                let chunk_transform = Transform {
                                    translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
                                    scale: Vec3::new(half_chunk_size * 2.0, half_chunk_size * 2.0, 1.0),
                                    ..Default::default()
                                };

                                world.spawn((
                                    SpriteBundle {
                                        sprite: default(),
                                        texture: metric_texture.clone(),
                                        transform: chunk_transform,
                                        ..Default::default()
                                    },
                                    ChunkComponent {
                                        coord: chunk_coord,
                                        owner: chunk_owner,
                                    },
                                ));

                                io.set_output(spawn::Output(Ok(())))
                            }),
                        }),
                    ],
                },
                ActionType {
                    name: "Despawn".to_owned(),
                    stages: vec![
                        ActionStage::Ecs(ActionStageEcs {
                            name: "FindAndDespawnEntity".to_owned(),
                            function: Box::new(|io: ActionStageIO<InputState>, world: &mut World| -> ActionStageIO<OutputState> {
                                let (input, io) = io.get_input::<despawn::FindAndDespawnEntityInput>();
                                let chunk_coord = input.chunk_coord;

                                let entity = world
                                    .query::<(Entity, &ChunkComponent)>()
                                    .iter(world)
                                    .find(|(_, chunk)| chunk.coord == chunk_coord)
                                    .map(|(entity, _)| entity);

                                if let Some(entity) = entity {
                                    world.entity_mut(entity).despawn_recursive();
                                    io.set_output(spawn::Output(Ok(())))
                                } else {
                                    io.set_output(spawn::Output(Err("Could not find chunk entity.".to_owned())))
                                }
                            
                            })
                        })
                    ],
                },
            ],
        },
    );
}