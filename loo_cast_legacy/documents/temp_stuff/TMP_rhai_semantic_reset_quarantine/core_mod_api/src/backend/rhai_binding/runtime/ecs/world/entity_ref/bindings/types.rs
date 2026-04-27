use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::bevy::prelude::{EntityMut as BevyEntityMut, EntityRef as BevyEntityRef};
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityRef {
    pub(crate) entity_ref: AccessCell<Scoped, BevyEntityRef<'static>>,
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityMut {
    pub(crate) entity_mut: AccessCell<Scoped, BevyEntityMut<'static>>,
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityWorldMut {
    pub(crate) entity_world_mut: AccessCell<Scoped, BevyEntityWorldMut<'static>>,
}
