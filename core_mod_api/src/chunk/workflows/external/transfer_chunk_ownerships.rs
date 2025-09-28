// Imports
use bevy::prelude::{ResMut, Entity, Query};

use crate::chunk::{components::Chunk, resources::ChunkManager, types::{ChunkCoord, ChunkOwnerId}};

// Items
pub struct TransferChunkOwnershipInput {
    pub new_chunk_owner_id: ChunkOwnerId,
    pub chunk_coord: ChunkCoord,
}

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_query: Query<'w, 's, (Entity, &'static mut Chunk)>,
    pub chunk_manager: ResMut<'w, ChunkManager>,
}

pub struct Input {
    pub inputs: Vec<TransferChunkOwnershipInput>,
}

pub struct Output {
    pub ownership_transfered_chunk_entities: Vec<Entity>
}

#[derive(Debug)]
pub enum Error {
    ChunkNotLoaded { chunk_coord: ChunkCoord },
}

// Core Functions
pub fn run_ecs(input: Input, main_access: MainAccess) -> Result<Output, Error> {
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