use crate::rhai_binding::runtime::ecs::query::bindings::types::EntityQuery as ScriptEntityQuery;
use crate::rhai_binding::runtime::ecs::query::internals::traits::EntityQueryApi;

core_mod_macros::reflect_extern_sub_module!(
    id = ecs::query,
    sub_modules = [],
    traits = [],
    types = [EntityQuery],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::query::EntityQuery,
    rust_type = ScriptEntityQuery,
    value_semantics = clone,
    method_functions = [
        ecs::query::EntityQuery::len,
        ecs::query::EntityQuery::is_empty,
        ecs::query::EntityQuery::to_array,
        ecs::query::EntityQuery::first_or_unit,
        ecs::query::EntityQuery::single,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::EntityQuery::len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query: &mut ScriptEntityQuery| query.len());
        engine.register_fn(name, |query: &mut ScriptEntityQuery| query.len());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::EntityQuery::is_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query: &mut ScriptEntityQuery| query.is_empty());
        engine.register_fn(name, |query: &mut ScriptEntityQuery| query.is_empty());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::EntityQuery::to_array,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptEntityQuery| query.to_array());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::EntityQuery::first_or_unit,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptEntityQuery| query.first_or_unit());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::EntityQuery::single,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptEntityQuery| query.single());
    },
);
