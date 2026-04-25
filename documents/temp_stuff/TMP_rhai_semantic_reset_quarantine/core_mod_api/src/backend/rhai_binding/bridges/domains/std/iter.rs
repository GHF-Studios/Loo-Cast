use crate::rhai_binding::runtime::std::iter::bindings::types::StringIter as ScriptStringIter;
use crate::rhai_binding::runtime::std::iter::internals::traits::StringIterApi;

core_engine_macros::reflect_extern_sub_module!(
    id = std::iter,
    sub_modules = [],
    traits = [],
    types = [StringIter],
    module_associated_functions = [],
);

core_engine_macros::reflect_extern_type!(
    id = std::iter::StringIter,
    rust_type = ScriptStringIter,
    value_semantics = clone,
    method_functions = [
        std::iter::StringIter::next,
        std::iter::StringIter::remaining_len,
        std::iter::StringIter::is_empty,
        std::iter::StringIter::collect_remaining,
    ],
);

core_engine_macros::reflect_extern_method_function!(
    id = std::iter::StringIter::next,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.next());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = std::iter::StringIter::remaining_len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |iter: &mut ScriptStringIter| iter.remaining_len());
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.remaining_len());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = std::iter::StringIter::is_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |iter: &mut ScriptStringIter| iter.is_empty());
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.is_empty());
    },
);

core_engine_macros::reflect_extern_method_function!(
    id = std::iter::StringIter::collect_remaining,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.collect_remaining());
    },
);
