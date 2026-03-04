use crate::bevy::prelude::World as BevyWorld;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};

#[repr(transparent)]
pub struct World {
    pub(crate) world: AccessCell<Scoped, BevyWorld>,
}
