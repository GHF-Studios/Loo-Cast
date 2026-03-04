use crate::rhai_binding::runtime::ecs::messages::bindings::types::MessageBatch as ScriptMessageBatch;
use crate::rhai_binding::runtime::ecs::messages::internals::traits::MessageBatchApi;

core_mod_macros::reflect_extern_sub_module!(
    id = ecs::messages,
    sub_modules = [],
    traits = [],
    types = [MessageBatch],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::messages::MessageBatch,
    rust_type = ScriptMessageBatch,
    value_semantics = clone,
    method_functions = [
        ecs::messages::MessageBatch::len,
        ecs::messages::MessageBatch::is_empty,
        ecs::messages::MessageBatch::to_array,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::messages::MessageBatch::len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |batch: &mut ScriptMessageBatch| batch.len());
        engine.register_fn(name, |batch: &mut ScriptMessageBatch| batch.len());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::messages::MessageBatch::is_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |batch: &mut ScriptMessageBatch| batch.is_empty());
        engine.register_fn(name, |batch: &mut ScriptMessageBatch| batch.is_empty());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::messages::MessageBatch::to_array,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |batch: &mut ScriptMessageBatch| batch.to_array());
    },
);
