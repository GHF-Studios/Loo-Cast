use bevy::ecs::world::EntityWorldMut;
use rhai::Dynamic;
use std::sync::Arc;

pub type TraitObjectUseParams = Dynamic;
pub type TraitObjectUseRefFn<T> = fn(&T, Dynamic) -> Dynamic;
pub type TraitObjectUseMutFn<T> = fn(&mut T, Dynamic) -> Dynamic;
pub type TraitObjectUseOwnedFn<T> = fn(T, Dynamic) -> Dynamic;

inventory::collect!(TraitObjectUseRefEntry);
pub struct TraitObjectUseRefEntry<T> {
    pub name: &'static str,
    pub ctor: TraitObjectUseRefFn,
}

inventory::collect!(TraitObjectUseRefEntry);
pub struct TraitObjectUseRefEntry<T> {
    pub name: &'static str,
    pub ctor: TraitObjectUseRefFn,
}

inventory::collect!(TraitObjectUseRefEntry);
pub struct TraitObjectUseRefEntry<T> {
    pub name: &'static str,
    pub ctor: TraitObjectUseRefFn,
}
