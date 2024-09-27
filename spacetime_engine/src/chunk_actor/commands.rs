use bevy::prelude::*;
use crate::chunk_actor::components::ChunkActor;
use crate::entity::structs::EntityPosition;
use crate::operations::structs::InstanceID;
use crate::operations::traits::*;
use crate::entity::operations::*;
use crate::chunk_actor::operations::*;
use crate::operations::utilities::*;

pub async fn spawn_chunk_actor() -> Result<InstanceID<ChunkActor>, String> {
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_result = run_op::<CreateEntity>(create_entity_args).await;
    let entity_id = match create_entity_result {
        CreateEntityResult::Ok { entity_id } => entity_id,
        CreateEntityResult::Err(_) => {
            return Err("Failed to create entity!".to_string());
        }
    };

    let upgrade_to_chunk_actor_args = UpgradeToChunkActorArgs {
        target_entity_id: entity_id,
        chunk_actor_start_chunk_id: InstanceID::new(1)
    };
    let upgrade_to_chunk_actor_result = run_op::<UpgradeToChunkActor>(upgrade_to_chunk_actor_args).await;
    let chunk_actor_id = match upgrade_to_chunk_actor_result {
        UpgradeToChunkActorResult::Ok { chunk_actor_id } => chunk_actor_id,
        UpgradeToChunkActorResult::Err(_) => {
            return Err(format!("Failed to upgrade entity '{}' to chunk actor!", entity_id));
        }
    };

    Ok(chunk_actor_id)
}