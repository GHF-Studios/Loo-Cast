use crate::bevy::prelude::World as BevyWorld;
use crate::script::access::ScopedAccessHandle;

#[repr(transparent)]
pub struct World {
    pub(crate) world: ScopedAccessHandle<BevyWorld>,
}