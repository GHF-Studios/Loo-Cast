use bevy::ecs::world::EntityWorldMut;
use rhai::Dynamic;

pub trait InsertComponentFromDynamic: Sized + Send + Sync + 'static {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, params: Dynamic);
}
