use bevy::prelude::*;

use crate::action::{resources::ActionTypeModuleRegistry, target::ActionTypeModule};

// TODO: Create macro to define actions and their types in a more streamlined and natural way
// TODO: Instead of an Action Target Type, we should register an Action Module Type, and integrate that change everywhere that's related

pub fn initialize_action_type_module(action_type_module_registry: &mut ActionTypeModuleRegistry) {
    action_type_module_registry.register(
        ActionTypeModule {
            name: "Chunk".to_owned(),
            action_types: vec![
                spawn::create_action_type(),
                despawn::create_action_type(),
                transfer_ownership::create_action_type(),
            ],
        },
    );
}

pub mod spawn {
    use bevy::prelude::*;

    use crate::{action::{stage::{ActionStage, ActionStageAsync, ActionStageEcs}, stage_io::{ActionIO, InputState, OutputState}, types::ActionType}, chunk::{components::ChunkComponent, functions::chunk_pos_to_world, resources::ChunkManager}, config::statics::CONFIG};

    pub struct Input(pub SetupAndSpawnEntityInput);

    pub struct SetupAndSpawnEntityInput {
        pub chunk_coord: (i32, i32),
        pub chunk_owner: Option<Entity>,
        pub metric_texture: Handle<Image>,
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_action_type() -> ActionType {
        ActionType {
            name: "Spawn".to_owned(),
            validation: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<OutputState>, String> {
                let (action_input, io) = io.get_input::<Input>();

                let target = world.query::<&ChunkComponent>().iter(world).find(|chunk| chunk.coord == action_input.0.chunk_coord);

                if target.is_some() {
                    return Err("Action validation error: Cannot spawn an already loaded chunk.".to_owned());
                }
                
                Ok(io.set_output(action_input.0))
            }),
            stages: vec![
                ActionStage::Ecs(ActionStageEcs {
                    name: "SetupAndSpawnEntity".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<SetupAndSpawnEntityInput>();
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

                        let mut chunk_manager = world.resource_mut::<ChunkManager>();
                        chunk_manager.loaded_chunks.insert(chunk_coord);
                        if let Some(chunk_owner) = chunk_owner {
                            chunk_manager.owned_chunks.insert(chunk_coord, chunk_owner);
                        }

                        io.set_output(Output(Ok(())))
                    }),
                }),
            ],
        }
    }
}

pub mod despawn {
    use bevy::prelude::*;

    use crate::{action::{stage::{ActionStage, ActionStageEcs}, stage_io::{ActionIO, InputState, OutputState}, types::ActionType}, chunk::{components::ChunkComponent, resources::ChunkManager}};

    pub struct Input(pub FindAndDespawnEntityInput);

    pub struct FindAndDespawnEntityInput {
        pub chunk_coord: (i32, i32),
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_action_type() -> ActionType {
        ActionType {
            name: "Despawn".to_owned(),
            validation: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<OutputState>, String> {
                let (action_input, io) = io.get_input::<Input>();

                let target = world.query::<&ChunkComponent>().iter(world).find(|chunk| chunk.coord == action_input.0.chunk_coord);

                if target.is_none() {
                    return Err("Action validation error: Cannot despawn an already unloaded chunk.".to_owned());
                }
                
                Ok(io.set_output(action_input.0))
            }),
            stages: vec![
                ActionStage::Ecs(ActionStageEcs {
                    name: "FindAndDespawnEntity".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<FindAndDespawnEntityInput>();
                        let chunk_coord = input.chunk_coord;

                        let mut chunk_manager = world.resource_mut::<ChunkManager>();
                        chunk_manager.loaded_chunks.remove(&chunk_coord);
                        chunk_manager.owned_chunks.remove(&chunk_coord);

                        if let Some((entity, _)) = world
                            .query::<(Entity, &ChunkComponent)>()
                            .iter(world)
                            .find(|(_, chunk)| chunk.coord == chunk_coord) {
                            world.entity_mut(entity).despawn_recursive();
                            io.set_output(Output(Ok(())))
                        } else {
                            io.set_output(Output(Err("Could not find chunk entity.".to_owned())))
                        }
                    })
                })
            ],
        }
    }
}

pub mod transfer_ownership {
    use bevy::prelude::*;
    use bevy::ecs::system::SystemState;

    use crate::{action::{stage::{ActionStage, ActionStageEcs}, stage_io::{ActionIO, InputState, OutputState}, types::ActionType}, chunk::{components::ChunkComponent, resources::ChunkManager}};

    pub struct Input(pub FindChunkAndTransferOwnershipInput);
    
    pub struct FindChunkAndTransferOwnershipInput {
        pub chunk_coord: (i32, i32),
        pub new_chunk_owner: Entity
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_action_type() -> ActionType {
        ActionType {
            name: "TransferOwnership".to_owned(),
            validation: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<OutputState>, String> {
                let (action_input, io) = io.get_input::<Input>();

                let target = world.query::<&ChunkComponent>().iter(world).find(|chunk| chunk.coord == action_input.0.chunk_coord);

                if target.is_none() {
                    return Err("Action validation error: Cannot transfer ownership of an unloaded chunk.".to_owned());
                }
                
                Ok(io.set_output(action_input.0))
            }),
            stages: vec![
                ActionStage::Ecs(ActionStageEcs {
                    name: "FindChunkAndTransferOwnership".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>, world: &mut World| -> ActionIO<OutputState> {
                        let (input, io) = io.get_input::<FindChunkAndTransferOwnershipInput>();
                        let chunk_coord = input.chunk_coord;
                        let new_chunk_owner = input.new_chunk_owner;

                        let mut system_state: SystemState<(Query<&mut ChunkComponent>, ResMut<ChunkManager>)> = SystemState::new(world);
                        let (mut chunk_query, mut chunk_manager) = system_state.get_mut(world);

                        let mut chunk = chunk_query
                            .iter_mut()
                            .find(|chunk| chunk.coord == chunk_coord)
                            .unwrap_or_else(|| panic!("Failed to transfer ownership of chunk '{:?}': it is already despawned according to the Chunk Query", chunk_coord));

                        if chunk.owner.is_some() {
                            chunk_manager.owned_chunks.remove(&chunk_coord);
                        }
                        chunk.owner = Some(new_chunk_owner);
                        chunk_manager.owned_chunks.insert(chunk_coord, new_chunk_owner);

                        io.set_output(Output(Ok(())))
                    })
                })
            ],
        }
    }
}
