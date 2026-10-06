//! Loo Cast scenario bootstrap, authored fixtures, and scenario lifetime.
//!
//! ## Module map
//!
//! - `fixture`: Authored celestial scenario adapter over semantic construction/runtime
//!   facilities.
//! - `lighting`: Shared outdoor lighting for Loo Cast world compositions.
//! - `selection`: Startup world-bootstrap selector.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod fixture;
mod lighting;
mod selection;

use bevy::prelude::*;

/// Scenario lifetime membership, independent of transform inheritance. Canonical
/// bodies and observer-relative presentations must not inherit a scenario owner's
/// floating-origin translation merely because they are cleaned up together.
#[derive(Component)]
#[relationship(relationship_target = ScenarioMembers)]
pub(super) struct ScenarioMemberOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ScenarioMemberOf, linked_spawn)]
pub(super) struct ScenarioMembers(Vec<Entity>);

pub(in crate::game) use fixture::UniverseLandmarkIndex;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameScenario {
    #[default]
    Selection,
    Playground,
    CelestialFixture,
}

pub(super) struct GameScenarioPlugin;

impl Plugin for GameScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameScenario>();

        lighting::configure(app);
        selection::configure(app);
        fixture::configure(app);
    }
}
