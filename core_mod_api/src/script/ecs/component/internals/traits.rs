use bevy::ecs::world::EntityWorldMut;
use rhai::Dynamic;

use super::super::bindings::types::Component;

pub trait ComponentCtor: Send + Sync + 'static {
    fn create(args: Dynamic) -> Component;
}

pub trait InsertComponentFromDynamic: Sized + Send + Sync + 'static {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, params: Dynamic);
}
