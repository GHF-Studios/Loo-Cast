use crate::rhai_binding::runtime::ecs::resource::internals::statics::{is_registered_resource_type, registered_resource_type_ids};
use crate::rhai_binding::bridges::domains::bevy::ecs::catalog::resource_signatures::TYPE_PATH__SCRIPT_PROBE_RESOURCE;

core_mod_macros::reflect_extern_sub_module!(
    id = bevy::ecs::resource,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [script_probe_type_id, registered_type_ids, is_registered_type],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::resource::script_probe_type_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name)
            .set_into_module(parent_module, || -> String { TYPE_PATH__SCRIPT_PROBE_RESOURCE.to_string() });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::resource::registered_type_ids,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> rhai::Array {
            registered_resource_type_ids()
                .into_iter()
                .map(rhai::Dynamic::from)
                .collect::<rhai::Array>()
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::resource::is_registered_type,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name)
            .set_into_module(parent_module, |resource_type_id: rhai::ImmutableString| -> bool {
                is_registered_resource_type(resource_type_id.as_str())
            });
    },
);
