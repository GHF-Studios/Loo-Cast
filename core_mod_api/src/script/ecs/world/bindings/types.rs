use crate::bevy::prelude::World as BevyWorld;
use crate::rhai_binding::value_semantics::scoped_access::ScopedAccessHandle;

#[repr(transparent)]
pub struct World {
    pub(crate) world: ScopedAccessHandle<BevyWorld>,
}
