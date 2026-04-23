use rhai::{Dynamic, ImmutableString};

use crate::rhai_binding::runtime::ecs::component::bindings::types::Component as ScriptComponent;
use crate::rhai_binding::runtime::ecs::component::internals::statics::COMPONENT_CTOR_REGISTRY;

pub const TYPE_PATH__PLAYER: &str = "core_mod_api::player::components::Player";
pub const TYPE_PATH__CHUNK_ACTOR: &str = "core_mod_api::usf::chunk::components::ChunkActor";
pub const TYPE_PATH__CHUNK_LOADER: &str = "core_mod_api::usf::chunk::components::ChunkLoader";

core_mod_macros::reflect_extern_sub_module!(
    id = bevy::ecs::component,
    sub_modules = [],
    traits = [],
    types = [Component],
    module_associated_functions = [
        create_single,
        create_unit,
        registered_ids,
        is_registered,
        player_type_id,
        chunk_actor_type_id,
        chunk_loader_type_id,
    ],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::component::Component,
    rust_type = ScriptComponent,
    value_semantics = clone,
    method_functions = [],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::create_single,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, |component_id: ImmutableString, params: Dynamic| -> ScriptComponent {
            ScriptComponent::create_single((component_id.to_string().into(), params))
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::create_unit,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, |component_id: ImmutableString| -> ScriptComponent {
            ScriptComponent::create_single((component_id.to_string().into(), Dynamic::UNIT))
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::registered_ids,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> rhai::Array {
            COMPONENT_CTOR_REGISTRY()
                .keys()
                .map(|id| Dynamic::from(id.as_ref().to_string()))
                .collect::<rhai::Array>()
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::is_registered,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, |component_id: ImmutableString| -> bool {
            COMPONENT_CTOR_REGISTRY().contains_key(component_id.as_str())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::player_type_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> String { TYPE_PATH__PLAYER.to_string() });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::chunk_actor_type_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> String { TYPE_PATH__CHUNK_ACTOR.to_string() });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = bevy::ecs::component::chunk_loader_type_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        rhai::FuncRegistration::new(name).set_into_module(parent_module, || -> String { TYPE_PATH__CHUNK_LOADER.to_string() });
    },
);
