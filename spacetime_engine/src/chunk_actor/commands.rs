use bevy::prelude::*;
use crate::chunk_actor::components::ChunkActor;
use crate::entity::structs::EntityPosition;
use crate::operations::structs::InstanceID;
use crate::operations::traits::*;
use crate::entity::operations::*;
use crate::chunk_actor::operations::*;
use crate::operations::commands::*;
use crate::sprite_bundle::operations::*;

pub async fn spawn_chunk_actor(entity_position: EntityPosition) -> Result<InstanceID<ChunkActor>, String> {
    debug!("Creating entity ...");

    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_result = run_op::<CreateEntity>(create_entity_args).await;
    let entity_id = match create_entity_result {
        CreateEntityResult::Ok { entity_id } => entity_id,
        CreateEntityResult::Err(_) => {
            return Err("Failed to create entity!".to_string());
        }
    };

    debug!("Created entity '{}'", entity_id);
    debug!("Upgrading entity '{}' to chunk actor ...", entity_id);

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

    debug!("Upgraded entity '{}' to chunk actor '{}'", entity_id, chunk_actor_id);
    debug!("Upgrading entity '{}' to sprite bundle ...", entity_id);

    let upgrade_to_sprite_bundle_args = UpgradeToSpriteBundleArgs {
        target_entity_id: entity_id,
        ..Default::default()
    };
    let upgrade_to_sprite_bundle_result = run_op::<UpgradeToSpriteBundle>(upgrade_to_sprite_bundle_args).await;
    match upgrade_to_sprite_bundle_result {
        UpgradeToSpriteBundleResult::Ok(_) => {},
        UpgradeToSpriteBundleResult::Err(_) => {
            return Err(format!("Failed to upgrade entity '{}' to sprite bundle!", entity_id));
        }
    };
    
    debug!("Upgraded entity '{}' to sprite bundle", entity_id);

    Ok(chunk_actor_id)
}