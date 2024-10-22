use crate::core::structs::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    id: DynamicKey<Entity>,
}
impl SpacetimeEntity {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: DynamicKey::default(),
        }
    }

    pub fn id(&self) -> DynamicKey<Entity> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut DynamicKey<Entity> {
        &mut self.id
    }
}