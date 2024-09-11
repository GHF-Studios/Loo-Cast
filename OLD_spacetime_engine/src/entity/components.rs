use bevy::prelude::*;
use crate::operations::InstanceID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    id: InstanceID<Entity>,
}

impl SpacetimeEntity {
    pub fn new() -> Self {
        Self {
            id: InstanceID::default(),
        }
    }

    pub fn id(&self) -> InstanceID<Entity> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut InstanceID<Entity> {
        &mut self.id
    }
}