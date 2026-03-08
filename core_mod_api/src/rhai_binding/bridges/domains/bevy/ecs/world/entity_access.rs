use crate::rhai_binding::runtime::ecs::world::entity_ref::bindings::types::EntityWorldMut as ScriptEntityWorldMut;
use crate::rhai_binding::runtime::ecs::world::entity_ref::internals::traits::EntityWorldMutApi;

core_mod_macros::reflect_extern_sub_module!(
    id = bevy::ecs::world::entity_access,
    sub_modules = [],
    traits = [],
    types = [EntityWorldMut],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::world::entity_access::EntityWorldMut,
    rust_type = ScriptEntityWorldMut,
    value_semantics = scoped_mut,
    method_functions = [bevy::ecs::world::entity_access::EntityWorldMut::id],
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::world::entity_access::EntityWorldMut::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_world_mut: &mut ScriptEntityWorldMut| entity_world_mut.id());
        engine.register_fn(name, |entity_world_mut: &mut ScriptEntityWorldMut| entity_world_mut.id());
    },
);
