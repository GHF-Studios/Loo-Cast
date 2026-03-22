use rhai::FuncRegistration;

core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf::zone,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [time_scale_for_levels_above, time_scale_for_scale_indices],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::time_scale_for_levels_above,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |levels_above: i64| -> rhai::FLOAT {
            crate::usf::zone::time_scale_for_levels_above(levels_above) as rhai::FLOAT
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::time_scale_for_scale_indices,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |active_scale_index: i64, target_scale_index: i64| -> rhai::FLOAT {
            crate::usf::zone::time_scale_for_scale_indices(active_scale_index, target_scale_index) as rhai::FLOAT
        });
    },
);
