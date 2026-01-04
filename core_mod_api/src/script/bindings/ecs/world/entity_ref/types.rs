use std::sync::{Arc, Mutex};

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityRef {
    entity_mut: bevy::prelude::EntityRef<'static>
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityMut {
    entity_mut: Option<Arc<Mutex<bevy::prelude::EntityMut<'static>>>>
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityWorldMut {
    entity_world_mut: Option<Arc<Mutex<bevy::prelude::EntityWorldMut<'static>>>>
}
