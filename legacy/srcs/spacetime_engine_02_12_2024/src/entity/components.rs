use bevy::prelude::*;
use crate::structs::NumericID;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SpacetimeEntity {
    id: NumericID,
}
impl SpacetimeEntity {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: NumericID::default(),
        }
    }

    pub fn id(&self) -> NumericID {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut NumericID {
        &mut self.id
    }
}
impl LockingNode for Entity {}
impl LockingNode for SpacetimeEntity {}