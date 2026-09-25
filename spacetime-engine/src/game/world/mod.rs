//! Loo Cast world bootstrap and environment composition.

mod lighting;
mod fixture;
mod selection;

use bevy::prelude::*;

/// World lifetime membership, independent of transform inheritance. Canonical
/// bodies and observer-relative presentations must not inherit a scene owner's
/// floating-origin translation merely because they are cleaned up together.
#[derive(Component)]
#[relationship(relationship_target = WorldMembers)]
pub(super) struct WorldMemberOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = WorldMemberOf, linked_spawn)]
pub(super) struct WorldMembers(Vec<Entity>);

pub(in crate::game) use fixture::UniverseLandmarkIndex;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameWorld {
    #[default]
    Selection,
    Playground,
    CelestialFixture,
}

pub(super) struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameWorld>();

        lighting::configure(app);
        selection::configure(app);
        fixture::configure(app);
    }
}
