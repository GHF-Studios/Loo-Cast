use std::any::TypeId;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{action::{resources::ActionTargetTypeRegistry, stage_io::*, structs::*}, config::statics::CONFIG};

use super::{components::ChunkComponent, functions::chunk_pos_to_world};

pub mod spawn {
    use bevy::prelude::*;

    // Inputs
    pub struct GenerateMetricMapsInput {
        pub chunk_coord: (i32, i32),
        pub chunk_owner: Option<Entity>,
    }
    pub struct SetupAndSpawnEntityInput {
        pub chunk_coord: (i32, i32),
        pub chunk_owner: Option<Entity>,
        pub metric_texture: Handle<Image>,
    }
    pub struct Output(Result<(), String>);

    // Outputs
    
}

fn register(
    action_target_type_registry: &mut ResMut<ActionTargetTypeRegistry>
) {
    action_target_type_registry.register::<ChunkComponent>(
        ActionTargetType {
            name: "Chunk".to_owned(),
            type_id: TypeId::of::<ChunkComponent>(),
            actions_types: vec![
                ActionType {
                    name: "Spawn".to_owned(),
                    stages: vec![
                        ActionStage::NonEcs {
                            name: "GenerateMetricMaps".to_owned(),
                            function: Box::new(|io: ActionStageIO<InputState>| -> ActionStageIO<OutputState> {
                                let (input, io) = io.get_input::<spawn::GenerateMetricMapsInput>();
                                let chunk_coord = input.chunk_coord;
                                let chunk_owner = input.chunk_owner;

                                // Do fancy async GPU stuff
                                let metric_texture = todo!("We need to implement some way to allow this operation to be async, else we cannot really access the GPU");

                                io.set_output(spawn::SetupAndSpawnEntityInput {
                                    chunk_coord,
                                    chunk_owner,
                                    metric_texture,
                                })
                            }),
                        },
                        ActionStage::Ecs {
                            name: "SetupAndSpawnEntity".to_owned(),
                            function: Box::new(|io: ActionStageIO<InputState>, commands: &mut Commands| -> ActionStageIO<OutputState> {
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
                            
                                commands.spawn((
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

                                io.set_output(())
                            }),
                        },
                    ],
                },
            ],
        }
    );
}