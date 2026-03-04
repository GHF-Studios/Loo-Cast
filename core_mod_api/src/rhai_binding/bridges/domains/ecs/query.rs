use rhai::FuncRegistration;

use crate::rhai_binding::runtime::ecs::query::bindings::types::{
    Query as ScriptQuery, QueryData as ScriptQueryData, QueryDataTerm as ScriptQueryDataTerm, QueryFilter as ScriptQueryFilter,
};
use crate::rhai_binding::runtime::ecs::query::internals::traits::QueryApi;

core_mod_macros::reflect_extern_generic_definition!(
    id = "ecs::query::Query<TData, TFilter>",
    owner_kind = r#type,
    params = [TData, TFilter],
    bounds = [TData: [], TFilter: []],
    notes = [
        "Runtime query facade for pre-registered monomorphized query signatures.",
        "Descriptors can be composed dynamically but only registered signatures dispatch.",
    ],
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
    types = [Query, QueryDataTerm, QueryData, QueryFilter],
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
    id = ecs::query::QueryDataTerm,
    rust_type = ScriptQueryDataTerm,
    value_semantics = clone,
    constructor_functions = [
        ecs::query::QueryDataTerm::value,
        ecs::query::QueryDataTerm::ref_,
        ecs::query::QueryDataTerm::mut_,
    ],
    method_functions = [ecs::query::QueryDataTerm::type_id, ecs::query::QueryDataTerm::access,],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::query::QueryData,
    rust_type = ScriptQueryData,
    value_semantics = clone,
    constructor_functions = [ecs::query::QueryData::single, ecs::query::QueryData::tuple, ecs::query::QueryData::from_terms,],
    method_functions = [ecs::query::QueryData::len, ecs::query::QueryData::to_array,],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::query::QueryFilter,
    rust_type = ScriptQueryFilter,
    value_semantics = clone,
    constructor_functions = [
        ecs::query::QueryFilter::none,
        ecs::query::QueryFilter::require,
        ecs::query::QueryFilter::exclude,
        ecs::query::QueryFilter::from_sets,
    ],
    method_functions = [ecs::query::QueryFilter::with_types, ecs::query::QueryFilter::without_types,],
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
    id = ecs::query::QueryDataTerm::value,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::value(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryDataTerm::ref_,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::ref_(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryDataTerm::mut_,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::mut_(type_id)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryDataTerm::type_id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |term: &mut ScriptQueryDataTerm| term.type_id());
        engine.register_fn(name, |term: &mut ScriptQueryDataTerm| term.type_id());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryDataTerm::access,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |term: &mut ScriptQueryDataTerm| term.access());
        engine.register_fn(name, |term: &mut ScriptQueryDataTerm| term.access());
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryData::single,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryData {
            ScriptQueryData::single(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryData::tuple,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |types: rhai::Array| -> ScriptQueryData { ScriptQueryData::tuple(types) });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryData::from_terms,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |terms: rhai::Array| -> ScriptQueryData { ScriptQueryData::from_terms(terms) });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryData::len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query_data: &mut ScriptQueryData| query_data.len());
        engine.register_fn(name, |query_data: &mut ScriptQueryData| query_data.len());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryData::to_array,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query_data: &mut ScriptQueryData| query_data.to_array());
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryFilter::none,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> ScriptQueryFilter { ScriptQueryFilter::none() });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryFilter::require,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryFilter {
            ScriptQueryFilter::require(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryFilter::exclude,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryFilter {
            ScriptQueryFilter::exclude(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = ecs::query::QueryFilter::from_sets,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |with: rhai::Array, without: rhai::Array| -> ScriptQueryFilter {
            ScriptQueryFilter::from_sets(with, without)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryFilter::with_types,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query_filter: &mut ScriptQueryFilter| query_filter.with_types());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::query::QueryFilter::without_types,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query_filter: &mut ScriptQueryFilter| query_filter.without_types());
    },
);
