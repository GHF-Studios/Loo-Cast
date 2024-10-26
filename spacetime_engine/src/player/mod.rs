use bevy::prelude::*;

pub mod components;

pub(in crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
    }
}