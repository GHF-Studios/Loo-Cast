//! Physics integration owned by Spacetime Engine.

pub mod character;
pub mod collision_topology;
pub mod topology;

use std::time::Duration;

use avian3d::prelude::PhysicsPlugins;
use bevy::{prelude::*, time::Virtual};

use character::CharacterMovementPlugin;

/// Installs the physics backend and Spacetime Engine's physics-facing systems.
///
/// Avian owns collision detection and rigid-body infrastructure. Higher-level
/// gameplay semantics, such as character movement, remain engine code.
pub struct SpacetimePhysicsPlugin;

/// Prevent a slow render frame from recursively scheduling up to sixteen 64 Hz
/// FixedMain iterations and turning ordinary overload into a catch-up death spiral.
/// Under severe load, virtual simulation time intentionally slows instead.
const MAX_VIRTUAL_FRAME_DELTA: Duration = Duration::from_millis(50);

fn configure_overload_guard(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.set_max_delta(MAX_VIRTUAL_FRAME_DELTA);
}

impl Plugin for SpacetimePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<topology::SpatialSplitBox>()
            .add_systems(PreStartup, configure_overload_guard)
            .add_plugins(
                PhysicsPlugins::default()
                    .with_collision_hooks::<topology::SpatialTopologyCollisionHooks>(),
            )
            .add_plugins(CharacterMovementPlugin)
            .add_systems(PostUpdate, collision_topology::rebuild_clipped_colliders);
    }
}
