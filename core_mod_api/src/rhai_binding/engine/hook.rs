use crate::bevy::prelude::{Mut, World as BevyWorld};
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::value_semantics::scoped_access::{ScopedAccess, ScopedAccessHandle};
use crate::script::ecs::world::bindings::types::World;
use std::sync::{Arc, RwLock};

pub(in super::super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|source_world, mut engine: Mut<MainScriptEngineHandle>| {
            let engine = &mut engine.0;
            let hook_code = std::fs::read_to_string(&path).unwrap();
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();

            let world = std::mem::take(source_world);
            let world_raw_handle =
                ScopedAccessHandle(Arc::new(RwLock::new(ScopedAccess::new(Box::new(world)))));
            let world_binding = World {
                world: world_raw_handle.clone(),
            };
            let shared_world = rhai::Shared::new(world_binding);

            engine
                .call_fn::<()>(&mut scope, &ast, "main", (shared_world,))
                .unwrap();

            let mut world_raw_scoped = Arc::into_inner(world_raw_handle.0)
                .expect("World handle leaked or cloned")
                .into_inner()
                .expect("RwLock poisoned");
            let returned_world = world_raw_scoped
                .invalidate()
                .expect("World handle was already invalidated");
            *source_world = *returned_world;
        });
    }
}
