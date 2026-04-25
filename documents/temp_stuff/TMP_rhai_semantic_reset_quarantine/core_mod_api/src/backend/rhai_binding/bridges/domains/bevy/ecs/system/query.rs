use crate::rhai_binding::runtime::ecs::system::query::bindings::types::Query as ScriptQuery;
use crate::rhai_binding::runtime::ecs::system::query::internals::traits::QueryApi;

core_engine_macros::reflect_extern_generic_definition!(
    id = "bevy::ecs::system::Query<TData, TFilter>",
    owner_kind = r#type,
    params = [TData, TFilter],
    bounds = [TData: [bevy::ecs::query::QueryData], TFilter: [bevy::ecs::query::QueryFilter]],
    notes = [
        "Runtime query facade for pre-registered monomorphized query signatures.",
        "Dispatched query results are exposed as a cursor-style runtime iterator.",
        "Descriptors can be composed dynamically but only registered signatures dispatch.",
    ],
);

core_engine_macros::reflect_extern_generic_instantiation!(
    id = "bevy::ecs::system::Query<bevy::ecs::entity::Entity, core_mod_api::player::components::Player>",
    generic_id = "bevy::ecs::system::Query<TData, TFilter>",
    type_arguments = [bevy::ecs::entity::Entity, core_mod_api::player::components::Player],
    concrete_item_path = "bevy::ecs::system::Query",
    value_semantics = clone,
);

core_engine_macros::reflect_extern_type!(
    id = bevy::ecs::system::Query,
    rust_type = ScriptQuery,
    value_semantics = clone,
    method_functions = [
        bevy::ecs::system::Query::next,
        bevy::ecs::system::Query::remaining_len,
        bevy::ecs::system::Query::is_empty,
        bevy::ecs::system::Query::collect_remaining,
    ],
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Query::next,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptQuery| query.next());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Query::remaining_len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query: &mut ScriptQuery| query.remaining_len());
        engine.register_fn(name, |query: &mut ScriptQuery| query.remaining_len());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Query::collect_remaining,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptQuery| query.collect_remaining());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Query::is_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query: &mut ScriptQuery| query.is_empty());
        engine.register_fn(name, |query: &mut ScriptQuery| query.is_empty());
    },
);
