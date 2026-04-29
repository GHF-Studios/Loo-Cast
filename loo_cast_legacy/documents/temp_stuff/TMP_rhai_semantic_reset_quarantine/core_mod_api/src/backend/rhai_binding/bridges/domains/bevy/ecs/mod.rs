core_engine_macros::reflect_extern_sub_module!(
    id = bevy::ecs,
    sub_modules = [world, system, query, entity, bundle, message, resource, component],
    traits = [],
    types = [],
    module_associated_functions = [bridge_info],
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::bridge_info,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> String { "ecs bridge online".to_string() });
    },
);

pub mod bundle;
pub mod catalog;
pub mod component;
pub mod entity;
pub mod message;
pub mod query;
pub mod resource;
pub mod system;
pub mod world;
