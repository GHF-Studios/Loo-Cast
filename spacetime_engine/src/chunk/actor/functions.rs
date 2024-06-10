use bevy::prelude::*;

use crate::chunk::actor::components::*;
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;
use super::constants::*;

pub(in crate::chunk::actor) fn new_chunk_actor_entity(
    commands: &mut Commands,
    chunk_actor_id: ChunkActorID,
    chunk_id: ChunkID,
    world_position: Vec2,
) -> Entity {
    let new_chunk_actor_entity = commands
    .spawn(Transform::from_translation(world_position.extend(CHUNK_ACTOR_Z_INDEX)))
    .insert(ChunkActor::new(chunk_actor_id, chunk_id))
    // TODO: This should be moved to each location where a new chunk actor is created and needs custom components apart from the basics like a Transform and a ChunkActor component.
    //.insert(ProxyRigidBody::Dynamic)
    //.insert(ProxyCollider::Square { half_length: half_prop_size })
    //.insert(ProxyVelocity::linear(Vec2 { x: 0.0, y: 0.0 }))
    .id();

    new_chunk_actor_entity
}

pub(in crate::chunk::actor) fn convert_to_chunk_actor_entity(
    commands: &mut Commands,
    chunk_actor_id: ChunkActorID,
    chunk_id: ChunkID,
    target_entity_reference: Entity,
    eligible_entity_query: &mut Query<Entity, (With<Transform>, Without<ChunkActor>)>,
) -> Entity {
    match eligible_entity_query.get_mut(target_entity_reference) {
        Ok(eligible_entity) => {
            commands.entity(eligible_entity).insert(ChunkActor::new(chunk_actor_id, chunk_id)).id()
        },
        Err(_) => {
            panic!("Entity does not exist or does not have a Transform component.");
        }
    }
}