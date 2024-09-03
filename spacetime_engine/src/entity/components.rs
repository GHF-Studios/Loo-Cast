use bevy::prelude::*;
use crate::operations::InstanceID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    pub id: InstanceID<Entity>,
}