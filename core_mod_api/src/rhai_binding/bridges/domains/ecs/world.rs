use rhai::{Dynamic, FnPtr, Shared};
use std::any::TypeId as RustTypeId;

use crate::script::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::script::ecs::world::bindings::types::World;
use crate::script::ecs::world::internals::traits::WorldApi;

type ScriptWorld = Shared<World>;

core_mod_macros::reflect_extern_sub_module!(
    id = ecs::world,
    sub_modules = [],
    traits = [],
    types = [World],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::world::World,
    rust_type = ScriptWorld,
    value_semantics = scoped_mut,
    method_functions = [
        ecs::world::World::flush,
        ecs::world::World::commands,
        ecs::world::World::spawn_empty,
        ecs::world::World::spawn_single,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::flush,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>()], |_, args| {
            let world = &mut *args[0].write_lock::<ScriptWorld>().unwrap();
            world.flush();
            Ok(Dynamic::UNIT)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::commands,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[1].take().cast::<FnPtr>();
                let world = &mut *args[0].write_lock::<ScriptWorld>().unwrap();
                Ok(world.commands(ctx, callback))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::spawn_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[1].take().cast::<FnPtr>();
                let world = &mut *args[0].write_lock::<ScriptWorld>().unwrap();
                Ok(world.spawn_empty(ctx, callback))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::spawn_single,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [
                RustTypeId::of::<ScriptWorld>(),
                RustTypeId::of::<BundleTraitObject>(),
                RustTypeId::of::<FnPtr>(),
            ],
            |ctx, args| {
                let callback = args[2].take().cast::<FnPtr>();
                let bundle = args[1].take().cast::<BundleTraitObject>();
                let world = &mut *args[0].write_lock::<ScriptWorld>().unwrap();
                Ok(world.spawn_single(bundle, ctx, callback))
            },
        );
    },
);
