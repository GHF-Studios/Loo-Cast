use bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::chunk::structs::ChunkPosition;
use crate::entity::structs::EntityPosition;
use crate::operations::structs::InstanceID;
use crate::entity::operations::*;
use crate::chunk::operations::*;
use crate::operations::utilities::*;

pub async fn spawn_chunk(chunk_position: ChunkPosition) -> Result<InstanceID<Chunk>, String> {
    debug!("Creating entity ...");

    let entity_position: EntityPosition = chunk_position.into();
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_result = run_op::<CreateEntity>(create_entity_args).await;
    let entity_id = match create_entity_result {
        CreateEntityResult::Ok { entity_id } => entity_id,
        CreateEntityResult::Err(_) => {
            return Err("Failed to create entity!".to_string());
        }
    };

    debug!("Created entity '{}'", entity_id);
    debug!("Upgrading entity '{}' to chunk ...", entity_id);

    let upgrade_to_chunk_args = UpgradeToChunkArgs {
        target_entity_id: entity_id,
        chunk_position,
        chunk_owner: None
    };
    let upgrade_to_chunk_result = run_op::<UpgradeToChunk>(upgrade_to_chunk_args).await;
    let chunk_id = match upgrade_to_chunk_result {
        UpgradeToChunkResult::Ok { chunk_id } => chunk_id,
        UpgradeToChunkResult::Err(_) => {
            return Err(format!("Failed to upgrade entity '{}' to chunk!", entity_id));
        }
    };

    debug!("Upgraded entity '{}' to chunk '{}'", entity_id, chunk_id);

    Ok(chunk_id)
}