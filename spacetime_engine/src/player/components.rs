use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ProxyPlayer;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Player;