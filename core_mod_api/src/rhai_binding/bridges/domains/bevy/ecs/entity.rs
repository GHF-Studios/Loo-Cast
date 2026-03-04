use crate::bevy::ecs::entity::Entity as BevyEntity;

core_mod_macros::reflect_extern_sub_module!(
    id = bevy::ecs::entity,
    sub_modules = [],
    traits = [],
    types = [Entity],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::entity::Entity,
    rust_type = BevyEntity,
    value_semantics = clone,
    method_functions = [
        bevy::ecs::entity::Entity::index,
        bevy::ecs::entity::Entity::gen,
        bevy::ecs::entity::Entity::to_string,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::entity::Entity::index,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |e: &mut BevyEntity| e.index());
        engine.register_fn(name, |e: &mut BevyEntity| e.index());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::entity::Entity::gen,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |e: &mut BevyEntity| e.generation());
        engine.register_fn(name, |e: &mut BevyEntity| e.generation());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::entity::Entity::to_string,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |e: &mut BevyEntity| {
            format!("Entity(index={}, gen={})", e.index(), e.generation())
        });
    },
);
