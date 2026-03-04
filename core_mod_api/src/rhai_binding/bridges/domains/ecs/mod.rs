core_mod_macros::reflect_extern_top_level_module!(
    id = ecs,
    sub_modules = [world, commands, entities, bundle, query, messages],
    traits = [],
    types = [],
    module_associated_functions = [bridge_info],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = ecs::bridge_info,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> String { "ecs bridge online".to_string() });
    },
);

pub mod bundle;
pub mod catalog;
pub mod commands;
pub mod entities;
pub mod messages;
pub mod query;
pub mod world;
