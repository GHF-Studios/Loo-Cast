use rhai::FuncRegistration;

use crate::rhai_binding::runtime::ecs::system::query::bindings::types::{
    QueryData as ScriptQueryData, QueryDataTerm as ScriptQueryDataTerm, QueryFilter as ScriptQueryFilter,
};

core_mod_macros::reflect_extern_sub_module!(
    id = bevy::ecs::query,
    sub_modules = [],
    traits = [],
    types = [QueryDataTerm, QueryData, QueryFilter],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::query::QueryDataTerm,
    rust_type = ScriptQueryDataTerm,
    value_semantics = clone,
    constructor_functions = [
        bevy::ecs::query::QueryDataTerm::value,
        bevy::ecs::query::QueryDataTerm::ref_,
        bevy::ecs::query::QueryDataTerm::mut_,
        bevy::ecs::query::QueryDataTerm::value_t,
        bevy::ecs::query::QueryDataTerm::ref_t,
        bevy::ecs::query::QueryDataTerm::mut_t,
    ],
    method_functions = [bevy::ecs::query::QueryDataTerm::type_id, bevy::ecs::query::QueryDataTerm::access,],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::query::QueryData,
    rust_type = ScriptQueryData,
    value_semantics = clone,
    constructor_functions = [
        bevy::ecs::query::QueryData::single,
        bevy::ecs::query::QueryData::single_t,
        bevy::ecs::query::QueryData::tuple,
        bevy::ecs::query::QueryData::from_terms,
    ],
    method_functions = [bevy::ecs::query::QueryData::len, bevy::ecs::query::QueryData::to_array,],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::query::QueryFilter,
    rust_type = ScriptQueryFilter,
    value_semantics = clone,
    constructor_functions = [
        bevy::ecs::query::QueryFilter::none,
        bevy::ecs::query::QueryFilter::require,
        bevy::ecs::query::QueryFilter::require_t,
        bevy::ecs::query::QueryFilter::exclude,
        bevy::ecs::query::QueryFilter::exclude_t,
        bevy::ecs::query::QueryFilter::from_sets,
    ],
    method_functions = [bevy::ecs::query::QueryFilter::with_types, bevy::ecs::query::QueryFilter::without_types,],
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryDataTerm::value,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::value(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryDataTerm::ref_,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::ref_(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryDataTerm::mut_,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::mut_(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryDataTerm::value_t,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::value_t(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryDataTerm::ref_t,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::ref_t(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryDataTerm::mut_t,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryDataTerm {
            ScriptQueryDataTerm::mut_t(type_id)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::query::QueryDataTerm::type_id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |term: &mut ScriptQueryDataTerm| term.type_id());
        engine.register_fn(name, |term: &mut ScriptQueryDataTerm| term.type_id());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::query::QueryDataTerm::access,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |term: &mut ScriptQueryDataTerm| term.access());
        engine.register_fn(name, |term: &mut ScriptQueryDataTerm| term.access());
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryData::single,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryData {
            ScriptQueryData::single(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryData::single_t,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryData {
            ScriptQueryData::single_t(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryData::tuple,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |types: rhai::Array| -> ScriptQueryData { ScriptQueryData::tuple(types) });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryData::from_terms,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |terms: rhai::Array| -> ScriptQueryData { ScriptQueryData::from_terms(terms) });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::query::QueryData::len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |query_data: &mut ScriptQueryData| query_data.len());
        engine.register_fn(name, |query_data: &mut ScriptQueryData| query_data.len());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::query::QueryData::to_array,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query_data: &mut ScriptQueryData| query_data.to_array());
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryFilter::none,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> ScriptQueryFilter { ScriptQueryFilter::none() });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryFilter::require,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryFilter {
            ScriptQueryFilter::require(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryFilter::require_t,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryFilter {
            ScriptQueryFilter::require_t(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryFilter::exclude,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryFilter {
            ScriptQueryFilter::exclude(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryFilter::exclude_t,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |type_id: rhai::ImmutableString| -> ScriptQueryFilter {
            ScriptQueryFilter::exclude_t(type_id)
        });
    },
);

core_mod_macros::reflect_extern_constructor_function!(
    id = bevy::ecs::query::QueryFilter::from_sets,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |with: rhai::Array, without: rhai::Array| -> ScriptQueryFilter {
            ScriptQueryFilter::from_sets(with, without)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::query::QueryFilter::with_types,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query_filter: &mut ScriptQueryFilter| query_filter.with_types());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::query::QueryFilter::without_types,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |query_filter: &mut ScriptQueryFilter| query_filter.without_types());
    },
);
