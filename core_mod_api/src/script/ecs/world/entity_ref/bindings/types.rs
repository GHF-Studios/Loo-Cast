use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityRef {
    entity_ref: Option<Arc<bevy::prelude::EntityRef<'static>>>
}
impl EntityRef {
    pub(in crate::script) fn start_access<'w, 's>(source: bevy::prelude::EntityRef<'w>) -> Self {
        let static_source: bevy::prelude::EntityRef<'static> = unsafe {
            std::mem::transmute(source)
        };

        Self {
            entity_ref: Some(Arc::new(static_source)),
        }
    }

    pub(in crate::script) fn end_access<'w, 's>(mut self) -> bevy::prelude::EntityRef<'w> {
        let entity_ref = self.entity_ref.take().expect("Already cleaned up!");
        let entity_ref = Arc::into_inner(entity_ref).expect("Too many refs!");
        
        unsafe {
            std::mem::transmute(entity_ref)
        }
    }

    pub(in crate::script) fn raw_access<'w, 's>(&'_ self) -> &bevy::prelude::EntityRef<'w> {
        let entity_ref = self.entity_ref.as_ref().unwrap();

        unsafe {
            std::mem::transmute(entity_ref)
        }
    }
}
impl Drop for EntityRef {
    fn drop(&mut self) {
        if self.entity_ref.is_some() {
            panic!("This type should not be copied/cloned!")
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityMut {
    entity_mut: Option<Arc<Mutex<bevy::prelude::EntityMut<'static>>>>
}
impl EntityMut {
    pub(in crate::script) fn start_access<'w, 's>(source: bevy::prelude::EntityMut<'w>) -> Self {
        let static_source: bevy::prelude::EntityMut<'static> = unsafe {
            std::mem::transmute(source)
        };

        Self {
            entity_mut: Some(Arc::new(Mutex::new(static_source))),
        }
    }

    pub(in crate::script) fn end_access<'w, 's>(mut self) -> bevy::prelude::EntityMut<'w> {
        let entity_mut = self.entity_mut.take().expect("Already cleaned up!");
        let entity_mut = Arc::into_inner(entity_mut).expect("Too many refs!");
        let entity_mut = entity_mut.into_inner().unwrap();
        
        unsafe {
            std::mem::transmute(entity_mut)
        }
    }

    pub(in crate::script) fn raw_access<'w, 's>(&'_ self) -> MutexGuard<'_, bevy::prelude::EntityMut<'w>> {
        let entity_mut = self.entity_mut.as_ref().unwrap().lock().unwrap();

        unsafe {
            std::mem::transmute(entity_mut)
        }
    }
}
impl Drop for EntityMut {
    fn drop(&mut self) {
        if self.entity_mut.is_some() {
            panic!("This type should not be copied/cloned!")
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityWorldMut {
    entity_world_mut: Option<Arc<Mutex<bevy::prelude::EntityWorldMut<'static>>>>
}
impl EntityWorldMut {
    pub(in crate::script) fn start_access<'w, 's>(source: bevy::prelude::EntityWorldMut<'w>) -> Self {
        let static_source: bevy::prelude::EntityWorldMut<'static> = unsafe {
            std::mem::transmute(source)
        };

        Self {
            entity_world_mut: Some(Arc::new(Mutex::new(static_source))),
        }
    }

    pub(in crate::script) fn end_access<'w, 's>(mut self) -> bevy::prelude::EntityWorldMut<'w> {
        let entity_world_mut = self.entity_world_mut.take().expect("Already cleaned up!");
        let entity_world_mut = Arc::into_inner(entity_world_mut).expect("Too many refs!");
        let entity_world_mut = entity_world_mut.into_inner().unwrap();
        
        unsafe {
            std::mem::transmute(entity_world_mut)
        }
    }

    pub(in crate::script) fn raw_access<'w, 's>(&'_ self) -> MutexGuard<'_, bevy::prelude::EntityWorldMut<'w>> {
        let entity_world_mut = self.entity_world_mut.as_ref().unwrap().lock().unwrap();

        unsafe {
            std::mem::transmute(entity_world_mut)
        }
    }
}
impl Drop for EntityWorldMut {
    fn drop(&mut self) {
        if self.entity_world_mut.is_some() {
            panic!("This type should not be copied/cloned!")
        }
    }
}
