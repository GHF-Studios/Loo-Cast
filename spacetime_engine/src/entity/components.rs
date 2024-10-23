use crate::core::{structs::*, traits::LockingNode};
use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    id: DynamicID<Entity>,
}
impl SpacetimeEntity {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: DynamicID::default(),
        }
    }

    pub fn id(&self) -> DynamicID<Entity> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut DynamicID<Entity> {
        &mut self.id
    }
}
impl LockingNode for Entity {}
impl LockingNode for SpacetimeEntity {}