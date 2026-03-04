use crate::rhai_binding::runtime::ecs::query::bindings::types::{Query as ScriptQuery, QueryData as ScriptQueryData, QueryFilter as ScriptQueryFilter};
use crate::rhai_binding::runtime::ecs::query::internals::traits::QueryApi;

core_mod_macros::reflect_extern_generic_definition!(
    id = "ecs::query::Query<TData, TFilter>",
    owner_kind = r#type,
    params = [TData, TFilter],
    bounds = [TData: [], TFilter: []],
    notes = ["Runtime query facade carrying query data and query filter tokens."],
);

core_mod_macros::reflect_extern_generic_instantiation!(
    id = "ecs::query::Query<ecs::entities::Entity, player::components::Player>",
    generic_id = "ecs::query::Query<TData, TFilter>",
    type_arguments = [ecs::entities::Entity, player::components::Player],
    concrete_item_path = "ecs::query::Query",
    value_semantics = clone,
);

core_mod_macros::reflect_extern_sub_module!(
    id = ecs::query,
    sub_modules = [],
    traits = [],
    types = [Query, QueryData, QueryFilter],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::query::Query,
    rust_type = ScriptQuery,
    value_semantics = clone,
    method_functions = [
        ecs::query::Query::len,
        ecs::query::Query::is_empty,
        ecs::query::Query::to_array,
        ecs::query::Query::first_or_unit,
        ecs::query::Query::single,
        ecs::query::Query::try_single,
    ],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::query::QueryData,
    rust_type = ScriptQueryData,
    value_semantics = clone,
    constructor_functions = [ecs::query::QueryData::of],
    method_functions = [ecs::query::QueryData::id],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::query::QueryFilter,
    rust_type = ScriptQueryFilter,
    value_semantics = clone,
    constructor_functions = [ecs::query::QueryFilter::none, ecs::query::QueryFilter::of,],
    method_functions = [ecs::query::QueryFilter::id],
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::Query::len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query: &mut ScriptQuery| query.len());
        engine.register_fn(name, |query: &mut ScriptQuery| query.len());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::Query::is_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query: &mut ScriptQuery| query.is_empty());
        engine.register_fn(name, |query: &mut ScriptQuery| query.is_empty());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::Query::to_array,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptQuery| query.to_array());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::Query::first_or_unit,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptQuery| query.first_or_unit());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::Query::single,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptQuery| query.single());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::Query::try_single,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query: &mut ScriptQuery| query.try_single());
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryData::of,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, |id: rhai::ImmutableString| -> ScriptQueryData { ScriptQueryData::of(id) });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryFilter::none,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> ScriptQueryFilter { ScriptQueryFilter::none() });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryFilter::of,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, |id: rhai::ImmutableString| -> ScriptQueryFilter { ScriptQueryFilter::of(id) });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryData::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query_data: &mut ScriptQueryData| query_data.id());
        engine.register_fn(name, |query_data: &mut ScriptQueryData| query_data.id());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryFilter::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query_filter: &mut ScriptQueryFilter| query_filter.id());
        engine.register_fn(name, |query_filter: &mut ScriptQueryFilter| query_filter.id());
    },
);
