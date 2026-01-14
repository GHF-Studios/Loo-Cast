use rhai::{Dynamic, FnPtr, NativeCallContext};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::script::ecs::{
    bundle::bindings::types::Bundle, system::commands::bindings::types::Commands,
    component::internals::statics::COMPONENT_CTOR_REGISTRY,
    world::entity_ref::bindings::types::EntityWorldMut};

#[derive(Clone)]
pub struct ScopedAccess<T> {

}