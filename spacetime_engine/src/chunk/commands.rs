use bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::chunk::constants::HALF_CHUNK_SIZE_F32;
use crate::chunk::structs::ChunkPosition;
use crate::entity::structs::EntityPosition;
use crate::core::structs::DynamicID;
use crate::entity::operations::*;
use crate::chunk::operations::*;
use crate::operation::commands::*;
use crate::sprite_bundle::operations::*;

pub async fn spawn_chunk(chunk_position: ChunkPosition) -> Result<DynamicID<Chunk>, String> {
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

    let color = if (chunk_position.0.0 + chunk_position.0.1) % 2 == 0 {
        Color::srgba(0.25, 0.25, 0.25, 0.5)
    } else {
        Color::srgba(0.75, 0.75, 0.75, 0.5)
    };
    let rect = Some(Rect {
        min: Vec2::new(-HALF_CHUNK_SIZE_F32, -HALF_CHUNK_SIZE_F32),
        max: Vec2::new(HALF_CHUNK_SIZE_F32, HALF_CHUNK_SIZE_F32),
    });
    let upgrade_to_sprite_bundle_args = UpgradeToSpriteBundleArgs {
        target_entity_id: entity_id,
        sprite: Some(Sprite {
            color,
            rect,
            ..Default::default()
        }),
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

    Ok(chunk_id)
}