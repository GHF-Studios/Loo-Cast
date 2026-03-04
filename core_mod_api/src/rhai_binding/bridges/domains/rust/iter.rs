use crate::rhai_binding::runtime::rust::iter::bindings::types::StringIter as ScriptStringIter;
use crate::rhai_binding::runtime::rust::iter::internals::traits::StringIterApi;

core_mod_macros::reflect_extern_sub_module!(
    id = rust::iter,
    sub_modules = [],
    traits = [],
    types = [StringIter],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = rust::iter::StringIter,
    rust_type = ScriptStringIter,
    value_semantics = clone,
    method_functions = [
        rust::iter::StringIter::next,
        rust::iter::StringIter::remaining_len,
        rust::iter::StringIter::is_empty,
        rust::iter::StringIter::collect_remaining,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = rust::iter::StringIter::next,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.next());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = rust::iter::StringIter::remaining_len,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |iter: &mut ScriptStringIter| iter.remaining_len());
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.remaining_len());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = rust::iter::StringIter::is_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |iter: &mut ScriptStringIter| iter.is_empty());
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.is_empty());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = rust::iter::StringIter::collect_remaining,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |iter: &mut ScriptStringIter| iter.collect_remaining());
    },
);
