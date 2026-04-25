use core_mod_api::rhai::{Array, Dynamic};

fn dynamic_to_f64(value: Dynamic, context: &str, index: usize) -> f64 {
    if let Some(float) = value.clone().try_cast::<f64>() {
        return float;
    }
    if let Some(integer) = value.try_cast::<i64>() {
        return integer as f64;
    }
    panic!("ctx::math::vector::{context} expected numeric value at index {index}")
}

fn parse_vec3(value: Array, context: &str) -> [f64; 3] {
    if value.len() != 3 {
        panic!("ctx::math::vector::{context} expected vec3 array with length 3, got {}", value.len());
    }

    let mut iter = value.into_iter();
    let x = dynamic_to_f64(iter.next().unwrap(), context, 0);
    let y = dynamic_to_f64(iter.next().unwrap(), context, 1);
    let z = dynamic_to_f64(iter.next().unwrap(), context, 2);
    [x, y, z]
}

fn to_vec3_array(value: [f64; 3]) -> Array {
    vec![Dynamic::from_float(value[0]), Dynamic::from_float(value[1]), Dynamic::from_float(value[2])]
}

fn vec3(x: f64, y: f64, z: f64) -> Array {
    to_vec3_array([x, y, z])
}

fn add(lhs: Array, rhs: Array) -> Array {
    let lhs = parse_vec3(lhs, "add");
    let rhs = parse_vec3(rhs, "add");
    to_vec3_array([lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2]])
}

fn sub(lhs: Array, rhs: Array) -> Array {
    let lhs = parse_vec3(lhs, "sub");
    let rhs = parse_vec3(rhs, "sub");
    to_vec3_array([lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]])
}

fn scale(value: Array, scalar: f64) -> Array {
    let value = parse_vec3(value, "scale");
    to_vec3_array([value[0] * scalar, value[1] * scalar, value[2] * scalar])
}

fn dot(lhs: Array, rhs: Array) -> f64 {
    let lhs = parse_vec3(lhs, "dot");
    let rhs = parse_vec3(rhs, "dot");
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn length(value: Array) -> f64 {
    let value = parse_vec3(value, "length");
    (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt()
}

core_engine_macros::reflect_extern_sub_module!(
    id = ctx::math::vector,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [vec3, add, sub, scale, dot, length],
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::vector::vec3,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, vec3);
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::vector::add,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, add);
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::vector::sub,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, sub);
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::vector::scale,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, scale);
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::vector::dot,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, dot);
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = ctx::math::vector::length,
    registrator = |name: core_mod_api::rhai::ImmutableString, parent_module: &mut core_mod_api::rhai::Module| {
        core_mod_api::rhai::FuncRegistration::new(name).set_into_module(parent_module, length);
    },
);
