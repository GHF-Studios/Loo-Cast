use rhai::{Dynamic, FnPtr, NativeCallContext};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::script::bindings::ecs::{
    bundle::types::Bundle, system::commands::types::Commands,
    component::statics::COMPONENT_CTOR_REGISTRY,
    world::entity_ref::types::EntityWorldMut};

#[derive(Clone)]
#[repr(transparent)]
pub struct World {
    world: Option<Arc<Mutex<bevy::prelude::World>>>,
}
impl World {
    pub(in crate::script) fn start_access(source: bevy::prelude::World) -> Self {
        Self {
            world: Some(Arc::new(Mutex::new(source))),
        }
    }

    pub(in crate::script) fn end_access(mut self) -> bevy::prelude::World {
        let world = self.world.take().expect("Already cleaned up!");
        let world = Arc::into_inner(world).expect("Too many refs!");
        let world = world.into_inner().unwrap();

        world
    }

    pub(in crate::script) fn raw_access(&'_ self) -> MutexGuard<'_, bevy::prelude::World> {
        self.world.as_ref().unwrap().lock().unwrap()
    }

    pub fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let cmds = Commands::start_access(self.raw_access().commands());
        let (cmds, out): (Commands, Dynamic) = callback.call_within_context(&ctx, (cmds,)).unwrap();
        let _ = cmds.end_access();
        out
    }

    pub fn flush(&self) {
        self.raw_access().flush();
    }

    pub fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.raw_access();
        let ent = world.spawn_empty();
        let ent = EntityWorldMut::start_access(ent);
        let (ent, out): (EntityWorldMut, Dynamic) = callback.call_within_context(&ctx, (ent,)).unwrap();
        let _ = ent.end_access();
        out
    }

    pub fn spawn(&self, bundle: Bundle, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let ctor_registry = COMPONENT_CTOR_REGISTRY();
        let mut world = self.raw_access();
        let mut ent = world.spawn_empty();
        for (name, params) in bundle.0 {
            let ctor = ctor_registry.get(name.as_ref()).unwrap();
            ctor(&mut ent, params);
        }

        let ent = EntityWorldMut::start_access(ent);
        let (ent, out): (EntityWorldMut, Dynamic) = callback.call_within_context(&ctx, (ent,)).unwrap();
        let _ = ent.end_access();
        out
    }

    // My personal note book; not used anymore, idk lol. Like writing on the back of a printout.
    #[deprecated]
    pub fn spawn_named_entity(&self, _name: String) {
        // Irrelevant Notes
        // self.raw_access().add_observer(system);
        // self.raw_access().add_schedule(schedule);
        // self.raw_access().add_asset(asset);
        // self.raw_access().clear_all();
        // self.raw_access().despawn(entity);
        // self.raw_access().query();
        // self.raw_access().query_filtered();
        // self.raw_access().remove_resource();
        // self.raw_access().removed();
        // self.raw_access().run_schedule(label);
        // self.raw_access().run_system(id);
        // self.raw_access().spawn(bundle);
        // 
        // self.raw_access().archetypes();
        // self.raw_access().bundles();
        // self.raw_access().components();
        // self.raw_access().entity(entities);
        // self.raw_access().entity_mut(entities);
        // self.raw_access().entities();
        // self.raw_access().get(entity);
        // self.raw_access().get_mut(entity);
        // self.raw_access().get_entity(entities);
        // self.raw_access().get_entity_mut(entities);
        // self.raw_access().get_resource();
        // self.raw_access().get_resource_mut();
        // self.raw_access().init_resource();
        // self.raw_access().insert_resource(value);
        // self.raw_access().storages();
    }
}
impl Drop for World {
    fn drop(&mut self) {
        if self.world.is_some() {
            panic!("This type should not be copied/cloned!")
        }
    }
}