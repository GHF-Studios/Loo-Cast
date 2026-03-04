use crate::bevy::prelude::*;
use core_mod_macros::component_ctor;
use rhai::Dynamic;

use crate::rhai_binding::runtime::ecs::component::internals::traits::InsertComponentFromDynamic;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component_ctor]
pub struct Player;
impl Default for Player {
    fn default() -> Self {
        Player
    }
}
impl InsertComponentFromDynamic for Player {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, _params: Dynamic) {
        entity.insert(Player);
    }
}
