//! Procedural-world bootstrap composition.

mod bootstrap;
mod landmarks;
mod player;
mod scenery;

use bevy::prelude::*;

use super::GameWorld;

pub(in crate::game) use landmarks::UniverseLandmarkIndex;

pub(super) fn configure(app: &mut App) {
    app.init_resource::<landmarks::UniverseLandmarkIndex>();
    app.add_systems(
        OnEnter(GameWorld::Procedural),
        (
            bootstrap::spawn_procedural_world,
            scenery::spawn_universe_scenery,
            player::prepare_controlled_subject,
        )
            .chain(),
    )
    .add_systems(
        Update,
        scenery::audit_world_authority.run_if(in_state(GameWorld::Procedural)),
    );
}
