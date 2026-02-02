use crate::bevy::prelude::Entity as BevyEntity;
use rhai::{Array, Dynamic, FnPtr, NativeCallContext};

use crate::script::ecs::{bundle::bindings::types::Bundle, world::entity_ref::bindings::types::EntityRef};

pub trait EntityRefApi {
    fn id(&self) -> BevyEntity;
}

pub trait EntityMutApi {
    fn id(&self) -> BevyEntity;
}

pub trait EntityWorldMutApi {
    fn id(&self) -> BevyEntity;
}