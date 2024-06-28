use bevy::prelude::*;
use super::{components::ChunkLoader, constants::*, id::structs::*};

pub(super) fn new_chunk_loader_entity(
    world: &mut World, 
    chunk_loader_id: ChunkLoaderID,
    world_position: Vec2,
) -> Entity {
    world.spawn((
        Transform::from_translation(Vec3::new(world_position.x, world_position.y, 0.0)),
        ChunkLoader::new(chunk_loader_id, CHUNK_LOADER_LOAD_RADIUS),
    )).id()
}

pub(super) fn upgrade_to_chunk_loader_entity(
    world: &mut World, 
    chunk_loader_id: ChunkLoaderID, 
    target_entity_reference: Entity,
) -> Result<Entity, Entity> {
    
    let ineligible_entity_query_0 = world.query::<Entity, Without<Transform>>();
    let ineligible_entity_query_1 = world.query::<Entity, With<ChunkLoader>>();
    let eligible_entity_query = world.query::<Entity, (With<Transform>, Without<ChunkLoader>)>();

    if ineligible_entity_query_0.get(world, target_entity_reference).is_ok() {
        error!("Entity '{:?}' does not have a Transform component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if ineligible_entity_query_1.get(world, target_entity_reference).is_ok() {
        error!("Entity '{:?}' already has a ChunkLoader component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(eligible_entity) = eligible_entity_query.get_mut(world, target_entity_reference) {
        Ok(world.entity(eligible_entity).insert(ChunkLoader::new(chunk_loader_id, CHUNK_LOADER_LOAD_RADIUS)).id())
    } else {
        error!("Entity does not exist or does not have a Transform component.");

        Err(target_entity_reference)
    }
}