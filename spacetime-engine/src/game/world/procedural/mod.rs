//! Procedural-world bootstrap composition.

mod bootstrap;
pub(in crate::game) mod landmarks;
mod player;
mod scenery;
mod scale_stack;

use bevy::prelude::*;

use crate::spatial::UsfSpatialSet;

use super::GameWorld;

pub(super) fn configure(app: &mut App) {
    app.init_resource::<landmarks::UniverseLandmarkIndex>();
    app.add_systems(
        OnEnter(GameWorld::Procedural),
        (
            bootstrap::spawn_procedural_world,
            scenery::spawn_universe_scenery,
            player::prepare_player,
        )
            .chain(),
    )
    .add_systems(Update, scale_stack::sync_scale_stack)
    .add_systems(
        PostUpdate,
        scenery::sync_moon_coarse_proxy_visibility.after(UsfSpatialSet::ViewProjection),
    );
}

