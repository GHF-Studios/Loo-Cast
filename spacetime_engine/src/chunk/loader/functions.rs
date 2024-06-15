use bevy::prelude::*;
use super::{components::ChunkLoader, id::structs::*};

pub(super) fn new_chunk_loader_entity(
    commands: &mut Commands, 
    chunk_loader_id: ChunkLoaderID
) -> Entity {
    commands
        .spawn(Transform::default())
        .insert(ChunkLoader::new(chunk_loader_id, 4))
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
    if let Ok(_) = ineligible_entity_query_0.get(target_entity_reference) {
        error!("Entity '{:?}' does not have a Transform component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(_) = ineligible_entity_query_1.get(target_entity_reference) {
        error!("Entity '{:?}' already has a ChunkLoader component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(eligible_entity) = eligible_entity_query.get_mut(target_entity_reference) {
        return Ok(commands.entity(eligible_entity).insert(ChunkLoader::new(chunk_loader_id, 4)).id());
    } else {
        panic!("Entity does not exist or does not have a Transform component.");
    };
}

// TODO: Implement
pub(super) fn collect_chunk_loader_updates() {}

// TODO: Implement
pub(super) fn apply_chunk_loader_updates() {}