core_engine_macros::reflect_extern_sub_module!(
    id = ctx::math::scalar,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [add, sub, mul, div, pow, abs],
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::scalar::add,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, |lhs: f64, rhs: f64| -> f64 { lhs + rhs });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::scalar::sub,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, |lhs: f64, rhs: f64| -> f64 { lhs - rhs });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::scalar::mul,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, |lhs: f64, rhs: f64| -> f64 { lhs * rhs });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::scalar::div,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, |lhs: f64, rhs: f64| -> f64 {
            if rhs == 0.0 {
                panic!("ctx::math::scalar::div denied division by zero")
            }
            lhs / rhs
        });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::scalar::pow,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, |lhs: f64, rhs: f64| -> f64 { lhs.powf(rhs) });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::scalar::abs,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, |value: f64| -> f64 { value.abs() });
    },
);
