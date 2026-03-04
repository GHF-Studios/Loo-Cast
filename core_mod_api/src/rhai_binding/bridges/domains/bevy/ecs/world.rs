use rhai::{Dynamic, FnPtr, ImmutableString};
use std::any::TypeId as RustTypeId;

use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::system::query::bindings::types::{Query as ScriptQuery, QueryData as ScriptQueryData, QueryFilter as ScriptQueryFilter};
use crate::rhai_binding::runtime::ecs::world::bindings::types::World;
use crate::rhai_binding::runtime::ecs::world::entity_ref::bindings::types::{
    EntityMut as ScriptEntityMut, EntityRef as ScriptEntityRef,
};
use crate::rhai_binding::runtime::ecs::world::entity_ref::internals::traits::{EntityMutApi, EntityRefApi};
use crate::rhai_binding::runtime::ecs::world::internals::traits::WorldApi;
use crate::rhai_binding::runtime::std::iter::bindings::types::StringIter as ScriptStringIter;

type ScriptWorld = World;

core_mod_macros::reflect_extern_sub_module!(
    id = bevy::ecs::world,
    sub_modules = [entity_access],
    traits = [],
    types = [World, EntityRef, EntityMut],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::world::World,
    rust_type = ScriptWorld,
    value_semantics = scoped_mut,
    method_functions = [
        bevy::ecs::world::World::flush,
        bevy::ecs::world::World::commands,
        bevy::ecs::world::World::spawn_empty,
        bevy::ecs::world::World::spawn_single,
        bevy::ecs::world::World::query,
        bevy::ecs::world::World::query_filtered,
        bevy::ecs::world::World::write_probe_message,
        bevy::ecs::world::World::drain_probe_messages,
    ],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::world::EntityRef,
    rust_type = ScriptEntityRef,
    value_semantics = scoped_ref,
    method_functions = [bevy::ecs::world::EntityRef::id],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::world::EntityMut,
    rust_type = ScriptEntityMut,
    value_semantics = scoped_mut,
    method_functions = [bevy::ecs::world::EntityMut::id],
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::flush,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>()], |_, args| {
            let world = args[0].clone().cast::<ScriptWorld>();
            world.flush();
            Ok(Dynamic::UNIT)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::commands,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<FnPtr>()], |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let world = args[0].clone().cast::<ScriptWorld>();
            Ok(world.commands(ctx, callback))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::spawn_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<FnPtr>()], |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let world = args[0].clone().cast::<ScriptWorld>();
            Ok(world.spawn_empty(ctx, callback))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::spawn_single,
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
                let world = args[0].clone().cast::<ScriptWorld>();
                Ok(world.spawn_single(bundle, ctx, callback))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::query,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<ScriptQueryData>()], |_, args| {
            let data = args[1].take().cast::<ScriptQueryData>();
            let world = args[0].clone().cast::<ScriptWorld>();
            let query = world.query(data);
            Ok(Dynamic::from::<ScriptQuery>(query))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::query_filtered,
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
                let world = args[0].clone().cast::<ScriptWorld>();
                let query = world.query_filtered(data, filter);
                Ok(Dynamic::from::<ScriptQuery>(query))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::write_probe_message,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>(), RustTypeId::of::<ImmutableString>()], |_, args| {
            let payload = args[1].take().cast::<ImmutableString>();
            let world = args[0].clone().cast::<ScriptWorld>();
            world.write_probe_message(payload);
            Ok(Dynamic::UNIT)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::World::drain_probe_messages,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptWorld>()], |_, args| {
            let world = args[0].clone().cast::<ScriptWorld>();
            let messages = world.drain_probe_messages();
            Ok(Dynamic::from::<ScriptStringIter>(messages))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::EntityRef::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_ref: &mut ScriptEntityRef| entity_ref.id());
        engine.register_fn(name, |entity_ref: &mut ScriptEntityRef| entity_ref.id());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::EntityMut::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_mut: &mut ScriptEntityMut| entity_mut.id());
        engine.register_fn(name, |entity_mut: &mut ScriptEntityMut| entity_mut.id());
    },
);

pub mod entity_access;
