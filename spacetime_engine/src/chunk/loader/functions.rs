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
    eligible_entity_query: &mut Query<Entity, (With<Transform>, Without<ChunkLoader>)>
) -> Entity {
    match eligible_entity_query.get_mut(target_entity_reference) {
        Ok(eligible_entity) => {
            commands.entity(eligible_entity).insert(ChunkLoader::new(chunk_loader_id, 4)).id()
        },
        Err(_) => {
            panic!("Entity does not exist or does not have a Transform component.");
        }
    }
}