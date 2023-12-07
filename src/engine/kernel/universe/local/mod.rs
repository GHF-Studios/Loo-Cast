// Modules
pub mod id;

// Local imports

// Internal imports
use crate::engine::kernel::game::SimulationState;
use crate::engine::kernel::player::*;
use crate::engine::kernel::universe::chunk::data::*;
use crate::engine::kernel::universe::chunk::id::*;
use crate::engine::kernel::universe::chunk::metadata::*;
use crate::engine::kernel::universe::chunk::pos::*;
use crate::engine::kernel::universe::chunk::*;
use crate::engine::kernel::universe::entity::id::*;
use crate::engine::kernel::universe::entity::pos::*;
use crate::engine::kernel::universe::entity::*;
use crate::engine::kernel::universe::global::*;
use crate::engine::kernel::universe::*;
use crate::engine::kernel::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums

// Structs
pub struct LocalUniversePlugin;

#[derive(Debug)]
pub struct LocalUniverse {
    pub(in crate::engine::kernel::universe) id: LocalUniverseID,
    pub(in crate::engine::kernel::universe) previously_viewed_local_chunk_positions: Vec<ApparentLocalChunkPos>,
    pub(in crate::engine::kernel::universe) currently_viewed_local_chunk_positions: Vec<ApparentLocalChunkPos>,
    pub(in crate::engine::kernel::universe) newly_viewed_local_chunk_positions: Vec<ApparentLocalChunkPos>,
}

// Implementations
impl Plugin for LocalUniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Update Systems
            .add_systems(
                Update,
                (LocalUniverse::detect_local_chunks_system,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

impl Default for LocalUniverse {
    fn default() -> Self {
        Self::new(LocalUniverseID::default())
    }
}

impl LocalUniverse {
    pub fn new(id: LocalUniverseID) -> LocalUniverse {
        Self {
            id,
            previously_viewed_local_chunk_positions: Vec::new(),
            currently_viewed_local_chunk_positions: Vec::new(),
            newly_viewed_local_chunk_positions: Vec::new(),
        }
    }

    fn detect_local_chunks_system(
        player_transform_query: Query<&Transform, With<Player>>,
        universe_manager: Res<UniverseManager>,
    ) {
        let global_universe = match universe_manager.get_global_universe() {
            Some(global_universe) => global_universe,
            None => {
                return;
            }
        };
        let mut global_universe = match global_universe.lock() {
            Ok(global_universe) => global_universe,
            Err(_) => {
                return;
            }
        };

        let local_universe = match universe_manager.get_local_universe(LocalUniverseID::default()) {
            Some(local_universe) => local_universe,
            None => {
                return;
            }
        };
        let mut local_universe = match local_universe.lock() {
            Ok(local_universe) => local_universe,
            Err(_) => {
                return;
            }
        };

        let player_transform = player_transform_query.single(); 

        Self::gather_local_chunk_positions(&mut local_universe, player_transform);
        Self::process_local_chunk_positions(&mut global_universe, &mut local_universe);
    }

    fn gather_local_chunk_positions(
        local_universe: &mut LocalUniverse,
        local_universe_transform: &Transform,
    ) {
        if local_universe.previously_viewed_local_chunk_positions.len() > 0 {
            panic!("Chunk viewer's previously viewed chunk positions are not empty");
        }
        if local_universe.newly_viewed_local_chunk_positions.len() > 0 {
            panic!("Chunk viewer's newly viewed chunk positions are not empty");
        }

        let local_universe_local_entity_pos: LocalEntityPos =
            local_universe_transform.translation.into();
        let local_universe_apparent_local_chunk_position: ApparentLocalChunkPos =
            local_universe_local_entity_pos.into();
        let detected_chunk_positions =
            Self::get_chunks_in_range(&local_universe_apparent_local_chunk_position);
        let currently_viewed_chunk_positions = local_universe
            .currently_viewed_local_chunk_positions
            .clone();

        for currently_viewed_chunk_position in currently_viewed_chunk_positions {
            if !detected_chunk_positions.contains(&currently_viewed_chunk_position) {
                local_universe
                    .previously_viewed_local_chunk_positions
                    .push(currently_viewed_chunk_position);
            }
        }

        for detected_chunk_position in &detected_chunk_positions {
            if !local_universe
                .currently_viewed_local_chunk_positions
                .contains(detected_chunk_position)
            {
                local_universe
                    .newly_viewed_local_chunk_positions
                    .push(detected_chunk_position.clone());
            }
        }
    }

    fn process_local_chunk_positions(
        global_universe: &mut GlobalUniverse,
        local_universe: &mut LocalUniverse,
    ) {
        // Unload chunks that have exited the view
        let old_local_chunk_positions = local_universe
            .previously_viewed_local_chunk_positions
            .clone();

        for old_local_chunk_pos in &old_local_chunk_positions {
            let old_apparent_local_chunk_pos = old_local_chunk_pos.clone();
            let old_absolute_local_chunk_pos: AbsoluteLocalChunkPos = old_apparent_local_chunk_pos.clone().into();
            let old_absolute_local_chunk_pos_base10x10: (u8, u8) = old_absolute_local_chunk_pos.into();
            let old_chunk_id = match ChunkID::try_from(old_absolute_local_chunk_pos_base10x10) {
                Ok(old_chunk_id) => old_chunk_id,
                Err(error) => {
                    println!("Failed to create chunk id: {:?}", error);
                    continue;
                }
            };

            // get chunk
            let old_chunk = match global_universe.get_registered_chunk(&old_chunk_id.clone()) {
                Ok(old_chunk) => old_chunk,
                Err(error) => {
                    println!("Failed to get chunk: {:?}", error);
                    continue;
                }
            };

            let old_chunk = match old_chunk {
                Some(old_chunk) => old_chunk,
                None => {
                    println!("Failed to get chunk: {:?}", old_chunk_id);
                    continue;
                }
            };

            let old_chunk = match old_chunk.lock() {
                Ok(old_chunk) => old_chunk,
                Err(error) => {
                    println!("Failed to lock chunk: {:?}", error);
                    continue;
                }
            };

            let old_chunk_data = match GlobalUniverse::get_chunk_data(&old_chunk) {
                Ok(old_chunk_data) => old_chunk_data,
                Err(error) => {
                    println!("Failed to get chunk data: {:?}", error);
                    continue;
                }
            };

            let old_local_entity_ids = old_chunk_data.registered_entities.keys().cloned().collect::<Vec<LocalEntityID>>();

            for old_local_entity_id in old_local_entity_ids {
                let old_entity_id = EntityID::new(old_chunk_id.clone(), old_local_entity_id.clone());
                
                match global_universe.send_entity_operation_request(EntityOperationRequest {
                    operations: vec![
                        EntityOperation::Despawn {
                            id: old_entity_id.clone(),
                            success_callback: Box::new(|_, _| {}),
                            failure_callback: Box::new(|err, _| {
                                println!("Failed to despawn entity: {:?}", err);
                            }),
                        },
                        EntityOperation::UnloadData {
                            id: old_entity_id.clone(),
                            success_callback: Box::new(|_, _| {}),
                            failure_callback: Box::new(|err, _| {
                                println!("Failed to unload entity data: {:?}", err);
                            }),
                        },
                        EntityOperation::UnloadMetadata {
                            id: old_entity_id.clone(),
                            success_callback: Box::new(|_, _| {}),
                            failure_callback: Box::new(|err, _| {
                                println!("Failed to unload entity metadata: {:?}", err);
                            }),
                        },
                        EntityOperation::Unregister {
                            id: old_entity_id,
                            success_callback: Box::new(|_, _| {}),
                            failure_callback: Box::new(|err, _| {
                                println!("Failed to unregister entity: {:?}", err);
                            }),
                        },
                    ]
                }) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("Failed to send entity operation request: {:?}", error);
                        continue;
                    }
                }
            }

            match global_universe.send_chunk_operation_request(ChunkOperationRequest {
                operations: vec![
                    ChunkOperation::Despawn {
                        id: old_chunk_id.clone(),
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _| {
                            println!("Failed to despawn chunk: {:?}", err);
                        }),
                    },
                    ChunkOperation::UnloadData {
                        id: old_chunk_id.clone(),
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _| {
                            println!("Failed to unload chunk data: {:?}", err);
                        }),
                    },
                    ChunkOperation::UnloadMetadata {
                        id: old_chunk_id.clone(),
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _| {
                            println!("Failed to unload chunk metadata: {:?}", err);
                        }),
                    },
                    ChunkOperation::Unregister {
                        id: old_chunk_id,
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _| {
                            println!("Failed to unregister chunk: {:?}", err);
                        }),
                    },
                ],
            }) {
                Ok(_) => {}
                Err(error) => {
                    println!("Failed to send chunk operation request: {:?}", error);
                    continue;
                }
            }
        }

        local_universe
            .currently_viewed_local_chunk_positions
            .retain(|chunk_pos| !old_local_chunk_positions.contains(chunk_pos));

        local_universe
            .previously_viewed_local_chunk_positions
            .clear();

        // Load chunks that have entered the view
        let mut new_local_chunk_positions =
            local_universe.newly_viewed_local_chunk_positions.clone();

        for new_local_chunk_pos in &new_local_chunk_positions {
            let new_apparent_local_chunk_pos = new_local_chunk_pos.clone();
            let new_absolute_local_chunk_pos: AbsoluteLocalChunkPos = new_apparent_local_chunk_pos.clone().into();
            let new_absolute_local_chunk_pos_base10x10: (u8, u8) = new_absolute_local_chunk_pos.clone().into();
            let new_chunk_id = match ChunkID::try_from(new_absolute_local_chunk_pos_base10x10) {
                Ok(new_chunk_id) => new_chunk_id,
                Err(error) => {
                    println!("Failed to create chunk id: {:?}", error);
                    continue;
                }
            };

            let new_chunk_metadata = ChunkMetadata::new_root(new_absolute_local_chunk_pos);
            let new_chunk_data = ChunkData::new_node(new_apparent_local_chunk_pos.into());

            match global_universe.send_chunk_operation_request(ChunkOperationRequest {
                operations: vec![
                    ChunkOperation::Register {
                        id: new_chunk_id.clone(),
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _| {
                            println!("Failed to register chunk: {:?}", err);
                        }),
                    },
                    ChunkOperation::LoadMetadata {
                        id: new_chunk_id.clone(),
                        metadata: new_chunk_metadata,
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _, _| {
                            println!("Failed to load chunk metadata: {:?}", err);
                        }),
                    },
                    ChunkOperation::LoadData {
                        id: new_chunk_id.clone(),
                        data: new_chunk_data,
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _, _| {
                            println!("Failed to load chunk data: {:?}", err);
                        }),
                    },
                    ChunkOperation::Spawn {
                        id: new_chunk_id,
                        success_callback: Box::new(|_, _| {}),
                        failure_callback: Box::new(|err, _| {
                            println!("Failed to spawn chunk: {:?}", err);
                        }),
                    },
                ],
            }) {
                Ok(_) => {}
                Err(error) => {
                    println!("Failed to send chunk operation request: {:?}", error);
                    continue;
                }
            }
        }

        local_universe
            .currently_viewed_local_chunk_positions
            .append(&mut new_local_chunk_positions);

        local_universe.newly_viewed_local_chunk_positions.clear();
    }

    fn get_chunks_in_range(center: &ApparentLocalChunkPos) -> Vec<ApparentLocalChunkPos> {
        let mut chunks = Vec::new();
        let r = VIEW_RADIUS as i8;
        for x in (center.x - r)..=(center.x + r) {
            for y in (center.y - r)..=(center.y + r) {
                chunks.push(ApparentLocalChunkPos::new(x, y));
            }
        }
        chunks
    }

    pub fn get_id(&self) -> &LocalUniverseID {
        &self.id
    }
}

// Module Functions
