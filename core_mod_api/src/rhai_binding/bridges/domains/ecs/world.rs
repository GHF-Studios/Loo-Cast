use rhai::{Dynamic, FnPtr, ImmutableString, Shared};
use std::any::TypeId as RustTypeId;

use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::messages::bindings::types::MessageBatch as ScriptMessageBatch;
use crate::rhai_binding::runtime::ecs::query::bindings::types::{Query as ScriptQuery, QueryData as ScriptQueryData, QueryFilter as ScriptQueryFilter};
use crate::rhai_binding::runtime::ecs::world::bindings::types::World;
use crate::rhai_binding::runtime::ecs::world::internals::traits::WorldApi;

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
        ecs::world::World::query,
        ecs::world::World::query_filtered,
        ecs::world::World::write_probe_message,
        ecs::world::World::read_probe_messages,
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
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<FnPtr>()], |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let world = &mut *args[0].write_lock::<ScriptWorld>().unwrap();
            Ok(world.commands(ctx, callback))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::spawn_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<FnPtr>()], |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let world = &mut *args[0].write_lock::<ScriptWorld>().unwrap();
            Ok(world.spawn_empty(ctx, callback))
        });
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

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::query,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<ScriptQueryData>()], |_, args| {
            let data = args[1].take().cast::<ScriptQueryData>();
            let world = &*args[0].write_lock::<ScriptWorld>().unwrap();
            let query = world.query(data);
            Ok(Dynamic::from::<ScriptQuery>(query))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::query_filtered,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [
                RustTypeId::of::<ScriptWorld>(),
                RustTypeId::of::<ScriptQueryData>(),
                RustTypeId::of::<ScriptQueryFilter>(),
            ],
            |_, args| {
                let filter = args[2].take().cast::<ScriptQueryFilter>();
                let data = args[1].take().cast::<ScriptQueryData>();
                let world = &*args[0].write_lock::<ScriptWorld>().unwrap();
                let query = world.query_filtered(data, filter);
                Ok(Dynamic::from::<ScriptQuery>(query))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::write_probe_message,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<ImmutableString>()], |_, args| {
            let payload = args[1].take().cast::<ImmutableString>();
            let world = &*args[0].write_lock::<ScriptWorld>().unwrap();
            world.write_probe_message(payload);
            Ok(Dynamic::UNIT)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::world::World::read_probe_messages,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>()], |_, args| {
            let world = &*args[0].write_lock::<ScriptWorld>().unwrap();
            let messages = world.read_probe_messages();
            Ok(Dynamic::from::<ScriptMessageBatch>(messages))
        });
    },
);
