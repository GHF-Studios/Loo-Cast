use std::num::NonZeroU16;

use bevy::prelude::*;
use super::{components::ChunkLoader, constants::*, id::structs::*};

pub(super) fn new_chunk_loader_entity(
    commands: &mut Commands, 
    chunk_loader_id: ChunkLoaderID,
    world_position: Vec2,
) -> Entity {
    commands
        .spawn(Transform::from_translation(Vec3::new(world_position.x, world_position.y, 0.0)))
        .insert(ChunkLoader::new(chunk_loader_id, CHUNK_LOADER_LOAD_RADIUS))
        .id()
}

pub(super) fn upgrade_to_chunk_loader_entity(
    commands: &mut Commands, 
    chunk_loader_id: ChunkLoaderID, 
    target_entity_reference: Entity,
    ineligible_entity_query_0: &mut Query<Entity, Without<Transform>>,
    ineligible_entity_query_1: &mut Query<Entity, With<ChunkLoader>>,
    eligible_entity_query: &mut Query<Entity, (With<Transform>, Without<ChunkLoader>)>,
) -> Result<Entity, Entity> {
    if ineligible_entity_query_0.get(target_entity_reference).is_ok() {
        error!("Entity '{:?}' does not have a Transform component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if ineligible_entity_query_1.get(target_entity_reference).is_ok() {
        error!("Entity '{:?}' already has a ChunkLoader component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(eligible_entity) = eligible_entity_query.get_mut(target_entity_reference) {
        Ok(commands.entity(eligible_entity).insert(ChunkLoader::new(chunk_loader_id, CHUNK_LOADER_LOAD_RADIUS)).id())
    } else {
        error!("Entity does not exist or does not have a Transform component.");

        Err(target_entity_reference)
    }
}