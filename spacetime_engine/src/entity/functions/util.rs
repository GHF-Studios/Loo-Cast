use bevy::prelude::*;
use crate::entity::{components::SpacetimeEntity, id::structs::EntityID};

pub(in crate) fn create_entity(
    world: &mut World,
    entity_id: EntityID,
    world_position: Vec2,
) {
    world.spawn((
        Transform::from_translation(world_position.extend(0.0)),
        SpacetimeEntity {
            id: entity_id,
        },
    ));
}

pub(in crate) fn destroy_entity(
    world: &mut World,
    entity_reference: Entity,
) {
    world.despawn(entity_reference);
}