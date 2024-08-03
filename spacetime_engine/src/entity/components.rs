use bevy::prelude::*;
use super::id::structs::EntityID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    pub id: EntityID,
}