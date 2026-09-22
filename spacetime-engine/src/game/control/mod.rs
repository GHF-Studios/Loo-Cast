//! Semantic and local runtime control authority.
//!
//! Control authority is orthogonal to constituency, manifestation and
//! locomotion. A semantic controller may control another semantic entity while
//! local input is routed to one runtime manifestation of that subject.

use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalController;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalControlSubject;

/// Semantic control relationship. The relationship lives on the controlled
/// semantic subject and points at the semantic controller.
#[derive(Component, Debug)]
#[relationship(relationship_target = ControlledSubjects)]
pub struct ControlledBy(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = ControlledBy)]
pub struct ControlledSubjects(Vec<Entity>);

impl ControlledSubjects {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Entity> + '_ {
        self.0.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Stable controller-adapter extension points inside `RunFixedMainLoop`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlSet {
    /// Sample device/AI/network/replay controller state into generic intent.
    Sample,
    /// Convert controller actions into semantic locomotion/control requests.
    Request,
    /// Produce detailed character-motor intent after locomotion is resolved.
    CharacterIntent,
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocalController>()
            .register_type::<LocalControlSubject>();
    }
}
