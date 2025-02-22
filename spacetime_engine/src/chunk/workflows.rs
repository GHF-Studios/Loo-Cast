use crate::workflow::{resources::WorkflowTypeModuleRegistry, types::WorkflowTypeModule};

pub fn initialize_workflow_type_module(workflow_type_module_registry: &mut WorkflowTypeModuleRegistry) {
    workflow_type_module_registry.register(
        WorkflowTypeModule {
            name: "Chunk".to_owned(),
            workflow_types: vec![
                spawn::create_workflow_type(),
                despawn::create_workflow_type(),
                transfer_ownership::create_workflow_type(),
            ],
        },
    );
}

pub mod spawn {
    use bevy::prelude::*;

    use crate::{workflow::{stage::{WorkflowStage, WorkflowStageEcs}, io::{WorkflowIO, InputState, OutputState}, types::{WorkflowType, RawWorkflowData}}, chunk::{components::ChunkComponent, functions::chunk_pos_to_world, resources::ChunkManager}, config::statics::CONFIG};

    pub struct Input(pub SetupAndSpawnEntityInput);

    pub struct SetupAndSpawnEntityInput {
        pub chunk_coord: (i32, i32),
        pub chunk_owner: Option<Entity>,
        pub metric_texture: Handle<Image>,
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_workflow_type() -> WorkflowType {
        WorkflowType {
            name: "Spawn".to_owned(),
            primary_validation: Box::new(|io: WorkflowIO<InputState>| -> Result<WorkflowIO<InputState>, String> {
                let (workflow_input, _) = io.get_input::<Input>();
                let stage_input = workflow_input.0;

                Ok(WorkflowIO::new_input(RawWorkflowData::new(stage_input)))
            }),
            secondary_validation: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> Result<WorkflowIO<InputState>, String> {
                let stage_input = io.get_input_ref::<SetupAndSpawnEntityInput>();

                let target = world.query::<&ChunkComponent>().iter(world).find(|chunk| chunk.coord == stage_input.chunk_coord);

                if target.is_some() {
                    return Err("Workflow validation error: Cannot spawn an already loaded chunk.".to_owned());
                }
                
                Ok(io)
            }),
            stages: vec![
                WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "SetupAndSpawnEntity".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
                        let (input, io) = io.get_input::<SetupAndSpawnEntityInput>();
                        let chunk_coord = input.chunk_coord;
                        let chunk_owner = input.chunk_owner;
                        let metric_texture = input.metric_texture;

                        let default_chunk_z = CONFIG.get::<f32>("chunk/default_z");

                        let chunk_transform = Transform {
                            translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
                            ..Default::default()
                        };

                        world.spawn((
                            SpriteBundle {
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

                        io.set_output(RawWorkflowData::new(Output(Ok(()))))
                    }),
                }),
            ],
        }
    }
}

pub mod despawn {
    use bevy::prelude::*;

    use crate::{workflow::{stage::{WorkflowStage, WorkflowStageEcs}, io::{WorkflowIO, InputState, OutputState}, types::{WorkflowType, RawWorkflowData}}, chunk::{components::ChunkComponent, resources::ChunkManager}};

    pub struct Input(pub FindAndDespawnEntityInput);

    pub struct FindAndDespawnEntityInput {
        pub chunk_coord: (i32, i32),
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_workflow_type() -> WorkflowType {
        WorkflowType {
            name: "Despawn".to_owned(),
            primary_validation: Box::new(|io: WorkflowIO<InputState>| -> Result<WorkflowIO<InputState>, String> {
                let (workflow_input, _) = io.get_input::<Input>();
                let stage_input = workflow_input.0;

                Ok(WorkflowIO::new_input(RawWorkflowData::new(stage_input)))
            }),
            secondary_validation: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> Result<WorkflowIO<InputState>, String> {
                let stage_input = io.get_input_ref::<FindAndDespawnEntityInput>();

                let target = world.query::<&ChunkComponent>().iter(world).find(|chunk| chunk.coord == stage_input.chunk_coord);

                if target.is_none() {
                    return Err("Workflow validation error: Cannot despawn an already unloaded chunk.".to_owned());
                }
                
                Ok(io)
            }),
            stages: vec![
                WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "FindAndDespawnEntity".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
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
                            io.set_output(RawWorkflowData::new(Output(Ok(()))))
                        } else {
                            io.set_output(RawWorkflowData::new(Output(Err("Could not find chunk entity.".to_owned()))))
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

    use crate::{workflow::{stage::{WorkflowStage, WorkflowStageEcs}, io::{WorkflowIO, InputState, OutputState}, types::{WorkflowType, RawWorkflowData}}, chunk::{components::ChunkComponent, resources::ChunkManager}};

    pub struct Input(pub FindChunkAndTransferOwnershipInput);
    
    pub struct FindChunkAndTransferOwnershipInput {
        pub chunk_coord: (i32, i32),
        pub new_chunk_owner: Entity
    }

    pub struct Output(pub Result<(), String>);

    pub fn create_workflow_type() -> WorkflowType {
        WorkflowType {
            name: "TransferOwnership".to_owned(),
            primary_validation: Box::new(|io: WorkflowIO<InputState>| -> Result<WorkflowIO<InputState>, String> {
                let (workflow_input, _) = io.get_input::<Input>();
                let stage_input = workflow_input.0;

                Ok(WorkflowIO::new_input(RawWorkflowData::new(stage_input)))
            }),
            secondary_validation: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> Result<WorkflowIO<InputState>, String> {
                let stage_input = io.get_input_ref::<FindChunkAndTransferOwnershipInput>();

                let target = world.query::<&ChunkComponent>().iter(world).find(|chunk| chunk.coord == stage_input.chunk_coord);

                if target.is_none() {
                    return Err("Workflow validation error: Cannot transfer ownership of an unloaded chunk.".to_owned());
                }
                
                Ok(io)
            }),
            stages: vec![
                WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "FindChunkAndTransferOwnership".to_owned(),
                    function: Box::new(|io: WorkflowIO<InputState>, world: &mut World| -> WorkflowIO<OutputState> {
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

                        io.set_output(RawWorkflowData::new(Output(Ok(()))))
                    })
                })
            ],
        }
    }
}
