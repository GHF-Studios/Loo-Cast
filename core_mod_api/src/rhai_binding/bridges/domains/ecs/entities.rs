use rhai::Shared;

use crate::bevy::ecs::entity::Entity as BevyEntity;
use crate::rhai_binding::runtime::ecs::world::entity_ref::bindings::types::{
    EntityMut as ScriptEntityMut, EntityRef as ScriptEntityRef,
    EntityWorldMut as ScriptEntityWorldMut,
};
use crate::rhai_binding::runtime::ecs::world::entity_ref::internals::traits::{
    EntityMutApi, EntityRefApi, EntityWorldMutApi,
};

type SharedEntityRef = Shared<ScriptEntityRef>;
type SharedEntityMut = Shared<ScriptEntityMut>;
type SharedEntityWorldMut = Shared<ScriptEntityWorldMut>;

core_mod_macros::reflect_extern_sub_module!(
    id = ecs::entities,
    sub_modules = [],
    traits = [],
    types = [Entity, EntityRef, EntityMut, EntityWorldMut],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::entities::Entity,
    rust_type = BevyEntity,
    value_semantics = clone,
    method_functions = [
        ecs::entities::Entity::index,
        ecs::entities::Entity::gen,
        ecs::entities::Entity::to_string,
    ],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::entities::EntityRef,
    rust_type = SharedEntityRef,
    value_semantics = scoped_ref,
    method_functions = [ecs::entities::EntityRef::id],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::entities::EntityMut,
    rust_type = SharedEntityMut,
    value_semantics = scoped_mut,
    method_functions = [ecs::entities::EntityMut::id],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::entities::EntityWorldMut,
    rust_type = SharedEntityWorldMut,
    value_semantics = scoped_mut,
    method_functions = [ecs::entities::EntityWorldMut::id],
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::entities::Entity::index,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |e: &mut BevyEntity| e.index());
        engine.register_fn(name, |e: &mut BevyEntity| e.index());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::entities::Entity::gen,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |e: &mut BevyEntity| e.generation());
        engine.register_fn(name, |e: &mut BevyEntity| e.generation());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::entities::Entity::to_string,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_fn(name, |e: &mut BevyEntity| {
            format!("Entity(index={}, gen={})", e.index(), e.generation())
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::entities::EntityRef::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_ref: &mut SharedEntityRef| entity_ref.id());
        engine.register_fn(name, |entity_ref: &mut SharedEntityRef| entity_ref.id());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::entities::EntityMut::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_mut: &mut SharedEntityMut| entity_mut.id());
        engine.register_fn(name, |entity_mut: &mut SharedEntityMut| entity_mut.id());
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::entities::EntityWorldMut::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_world_mut: &mut SharedEntityWorldMut| {
            entity_world_mut.id()
        });
        engine.register_fn(name, |entity_world_mut: &mut SharedEntityWorldMut| {
            entity_world_mut.id()
        });
    },
);
