//! Procedural-world bootstrap composition.

mod bootstrap;
mod player;
mod scale_stack;

use bevy::prelude::*;

use super::GameWorld;

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        OnEnter(GameWorld::Procedural),
        (bootstrap::spawn_procedural_world, player::prepare_player).chain(),
    )
    .add_systems(Update, scale_stack::sync_scale_stack);
}

