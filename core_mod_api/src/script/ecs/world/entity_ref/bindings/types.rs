use bevy::prelude::{EntityRef as BevyEntityRef, EntityMut as BevyEntityMut, Entity as BevyEntity};
use bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;

use crate::script::core::internals::types::ScopedAccessHandle;

#[repr(transparent)]
pub struct EntityRef {
    pub(crate) entity_ref: ScopedAccessHandle<BevyEntityRef<'static>>
}

#[repr(transparent)]
pub struct EntityMut {
    pub(crate) entity_mut: ScopedAccessHandle<BevyEntityMut<'static>>
}

#[repr(transparent)]
pub struct EntityWorldMut {
    pub(crate) entity_world_mut: ScopedAccessHandle<BevyEntityWorldMut<'static>>
}
