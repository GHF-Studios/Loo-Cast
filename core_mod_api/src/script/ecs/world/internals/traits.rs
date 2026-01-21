use rhai::{Array, Dynamic, FnPtr, NativeCallContext};

use crate::script::ecs::bundle::bindings::types::Bundle;

pub trait WorldApi {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn flush(&self);
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn spawn_single(&self, bundle: Bundle, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    // fn spawn_batch(&self, bundles: Array, ctx: NativeCallContext, callback: FnPtr) -> Array;
    // fn query(&self, data: QueryData, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    // fn query_filtered(&self, data: QueryData, filter: QueryFilter, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
}

// Notes/Scratchpad and old implementations below:
/*
impl World {
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

    pub fn spawn_batch(&self, bundles: rhai::Array, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let ctor_registry = COMPONENT_CTOR_REGISTRY();
        let mut world = self.raw_access();
        let mut results = rhai::Array::with_capacity(bundles.len());

        for bundle_dyn in bundles {
            // Downcast each item to a Bundle
            let bundle = bundle_dyn.cast::<Bundle>();

            // Spawn an empty entity
            let mut ent = world.spawn_empty();

            // Insert each component via the dynamic constructor
            for (name, params) in bundle.0 {
                let ctor = ctor_registry.get(&name).expect("Component constructor not found");
                ctor(&mut ent, params);
            }

            // Hand over to Rhai callback for customization
            let ent = EntityWorldMut::start_access(ent);
            let (ent, out): (EntityWorldMut, Dynamic) =
                callback.call_within_context(&ctx, (ent,)).expect("Callback failed");
            let _ = ent.end_access();

            results.push(out);
        }

        Dynamic::from(results)
    }

    // pub fn query(&self, data: QueryData, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
    // }

    // pub fn query_filtered(&self, data: QueryData, filter: QueryFilter, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
    // }

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
*/