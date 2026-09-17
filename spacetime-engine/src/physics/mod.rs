//! Physics integration owned by Spacetime Engine.

pub mod character;
pub mod collision_topology;
pub mod topology;

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
        app.register_type::<topology::SpatialSplitBox>()
            .add_plugins(
                PhysicsPlugins::default()
                    .with_collision_hooks::<topology::SpatialTopologyCollisionHooks>(),
            )
            .add_plugins(CharacterMovementPlugin)
            .add_systems(PostUpdate, collision_topology::rebuild_clipped_colliders);
    }
}
