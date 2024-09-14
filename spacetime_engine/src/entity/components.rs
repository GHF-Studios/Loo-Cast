use crate::operations::structs::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    id: InstanceID<Entity>,
}
impl SpacetimeEntity {
    #[allow(clippy::new_without_default)]
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