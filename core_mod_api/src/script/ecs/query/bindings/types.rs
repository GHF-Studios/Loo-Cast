use crate::bevy::prelude::Entity as BevyEntity;

#[derive(Clone, Default)]
pub struct EntityQuery {
    pub(crate) entities: Vec<BevyEntity>,
}
