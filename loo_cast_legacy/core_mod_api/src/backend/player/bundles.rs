use crate::bevy::prelude::*;
use crate::rhai_binding::meta::abstract_::trait_identity::GetTypeId;

use super::components::Player;

#[derive(Bundle, Reflect)]
pub struct PlayerBundle {
    pub player: Player,
    pub transform: Transform,
    pub name: Name,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            transform: Transform::default(),
            name: Name::new("player"),
        }
    }
}
impl PlayerBundle {
    pub fn new_default() -> Self {
        Self::default()
    }

    pub fn test_print(&self) {
        println!("PlayerBundle test_print method successfully called!");
    }
}
impl GetTypeId for PlayerBundle {
    const TYPE_ID: &'static str = "core_mod_api::player::bundles::PlayerBundle";
}
