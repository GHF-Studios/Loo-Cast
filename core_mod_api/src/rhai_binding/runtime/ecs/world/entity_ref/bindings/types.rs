use crate::bevy::prelude::{EntityRef as BevyEntityRef, EntityMut as BevyEntityMut, Entity as BevyEntity};
use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};

#[repr(transparent)]
pub struct EntityRef {
    pub(crate) entity_ref: AccessCell<Scoped, BevyEntityRef<'static>>
}

#[repr(transparent)]
pub struct EntityMut {
    pub(crate) entity_mut: AccessCell<Scoped, BevyEntityMut<'static>>
}

#[repr(transparent)]
pub struct EntityWorldMut {
    pub(crate) entity_world_mut: AccessCell<Scoped, BevyEntityWorldMut<'static>>
}
