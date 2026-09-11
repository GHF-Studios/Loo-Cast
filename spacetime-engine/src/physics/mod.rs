//! Physics integration owned by Spacetime Engine.

pub mod character;

use avian3d::prelude::PhysicsPlugins;
use bevy::prelude::*;

use character::CharacterMovementPlugin;

/// Installs the physics backend and Spacetime Engine's physics-facing systems.
///
/// Avian owns collision detection and rigid-body infrastructure. Higher-level
/// gameplay semantics, such as character movement, remain engine code.
pub struct SpacetimePhysicsPlugin;

impl Plugin for SpacetimePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(CharacterMovementPlugin);
    }
}
