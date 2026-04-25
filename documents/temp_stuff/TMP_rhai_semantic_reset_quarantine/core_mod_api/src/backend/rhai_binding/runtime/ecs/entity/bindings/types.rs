use crate::bevy::prelude::*;

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityId {
    entity_id: Entity,
}
impl EntityId {
    pub fn from_raw(entity_id: Entity) -> Self {
        Self { entity_id }
    }

    pub fn to_raw(self) -> Entity {
        self.entity_id
    }
}
