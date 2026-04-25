use crate::rhai_binding::bridges::domains::bevy::ecs::catalog::message_signatures::TYPE_PATH__SCRIPT_PROBE_MESSAGE;
use crate::rhai_binding::runtime::ecs::message::internals::statics::{is_registered_message_type, registered_message_type_ids};

core_engine_macros::reflect_extern_sub_module!(
    id = bevy::ecs::message,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [script_probe_type_id, registered_type_ids, is_registered_type],
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::message::script_probe_type_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> String { TYPE_PATH__SCRIPT_PROBE_MESSAGE.to_string() });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::message::registered_type_ids,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> rhai::Array {
            registered_message_type_ids().into_iter().map(rhai::Dynamic::from).collect::<rhai::Array>()
        });
    },
);

core_engine_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::message::is_registered_type,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, |message_type_id: rhai::ImmutableString| -> bool {
            is_registered_message_type(message_type_id.as_str())
        });
    },
);
