use bevy::prelude::*;
use core_mod_macros::component_ctor;
use rhai::Dynamic;

use crate::script::bindings::core::traits::FromDynamic;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component_ctor]
pub struct Player;
impl Default for Player {
    fn default() -> Self {
        Player
    }
}
impl FromDynamic for Player {
    fn from_dynamic(params: Dynamic) -> Self {
        todo!()
    }
}
