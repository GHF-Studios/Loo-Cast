use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;
impl Default for Player {
    fn default() -> Self {
        Player
    }
}
