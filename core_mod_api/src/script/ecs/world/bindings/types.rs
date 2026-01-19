use bevy::prelude::World as BevyWorld;

use crate::script::core::internals::types::ScopedAccessHandle;

#[repr(transparent)]
pub struct World {
    pub(crate) world: ScopedAccessHandle<BevyWorld>,
}