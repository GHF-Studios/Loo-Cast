use crate::rhai_binding::runtime::ecs::world::entity_ref::bindings::types::EntityWorldMut as ScriptEntityWorldMut;
use crate::rhai_binding::runtime::ecs::world::entity_ref::internals::traits::EntityWorldMutApi;

core_engine_macros::reflect_extern_sub_module!(
    id = bevy::ecs::world::entity_access,
    sub_modules = [],
    traits = [],
    types = [EntityWorldMut],
    module_associated_functions = [],
);

core_engine_macros::reflect_extern_type!(
    id = bevy::ecs::world::entity_access::EntityWorldMut,
    rust_type = ScriptEntityWorldMut,
    value_semantics = scoped_mut,
    method_functions = [
        bevy::ecs::world::entity_access::EntityWorldMut::id,
        bevy::ecs::world::entity_access::EntityWorldMut::insert_component,
        bevy::ecs::world::entity_access::EntityWorldMut::insert_components,
        bevy::ecs::world::entity_access::EntityWorldMut::remove_component,
    ],
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::entity_access::EntityWorldMut::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_world_mut: &mut ScriptEntityWorldMut| entity_world_mut.id());
        engine.register_fn(name, |entity_world_mut: &mut ScriptEntityWorldMut| entity_world_mut.id());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::entity_access::EntityWorldMut::insert_component,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(
            name,
            |entity_world_mut: &mut ScriptEntityWorldMut, component: crate::rhai_binding::runtime::ecs::component::bindings::types::Component| {
                entity_world_mut.insert_component(component);
            },
        );
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::entity_access::EntityWorldMut::remove_component,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |entity_world_mut: &mut ScriptEntityWorldMut, component_type_id: rhai::ImmutableString| {
            entity_world_mut.remove_component(component_type_id);
        });
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::entity_access::EntityWorldMut::insert_components,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |entity_world_mut: &mut ScriptEntityWorldMut, components: rhai::Array| {
            entity_world_mut.insert_components(components);
        });
    },
);
