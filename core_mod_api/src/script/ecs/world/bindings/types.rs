use crate::bevy::prelude::World as BevyWorld;
use core_mod_core::reflection::access::ScopedAccessHandle;

#[repr(transparent)]
pub struct World {
    pub(crate) world: ScopedAccessHandle<BevyWorld>,
}