// Imports
use bevy::prelude::{Res, ResMut, Entity, Query};

use crate::chunk::{components::Chunk, resources::ChunkManager, types::ChunkOwnerId};
use crate::usf::scale::Scale;
use crate::workflow::types::Outcome;

// Items
pub struct TransferChunkOwnershipInput {
    pub new_chunk_owner_id: ChunkOwnerId<S>,
    pub chunk_coord: (i32, i32),
}

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's, S: Scale> {
    chunk_query: Query<'w, 's, (Entity, &'static mut Chunk<S>)>,
    chunk_manager: ResMut<'w, ChunkManager<S>>,
}

pub struct Input<S: Scale> {
    inputs: Vec<TransferChunkOwnershipInput<S>>,
}

pub struct Output {
    ownership_transfered_chunk_entities: Vec<Entity>
}

pub enum Error {
    ChunkNotLoaded { chunk_coord: (i32, i32) },
}

// Core Functions
pub fn run_ecs<S: Scale>(input: Input<S>, main_access: MainAccess<S>) -> Result<Output<S>, Error<S>> {
    let mut chunk_query = main_access.chunk_query;
    let mut chunk_manager = main_access.chunk_manager;

    let mut chunk_entities = Vec::new();

    for input in input.inputs {
        let new_chunk_owner_id = input.new_chunk_owner_id;
        let chunk_coord = input.chunk_coord;

        if let Some((entity, mut chunk)) = chunk_query.iter_mut().find(|(_, chunk)| chunk.coord == chunk_coord) {
            if chunk.owner_id.is_some() {
                chunk_manager.owned_chunks.remove(&chunk_coord);
            }
            chunk.owner_id = Some(new_chunk_owner_id.clone());
            chunk_manager.owned_chunks.insert(chunk_coord, new_chunk_owner_id);

            chunk_entities.push(entity);
        } else {
            return Err(Error::ChunkNotLoaded { chunk_coord });
        }
    }

    Ok(Output {
        ownership_transfered_chunk_entities: chunk_entities
    })
}