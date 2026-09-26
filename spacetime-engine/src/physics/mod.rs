//! Physics integration owned by Spacetime Engine.

pub mod chart;
pub mod character;
pub mod collision_topology;
pub mod gravity;
mod hull;
pub mod topology;

pub use hull::{DetailedBodyCollision, PhysicalBoxHull};

use std::time::Duration;

use avian3d::{
    collider_tree::update_moved_collider_aabbs,
    prelude::{Collider, Gravity, PhysicsPlugins},
};
use bevy::{prelude::*, time::Virtual};

use crate::spatial::UsfSpatialSet;

use character::CharacterMovementPlugin;
use gravity::GravityPlugin;

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
        app.register_type::<PhysicalBoxHull>()
            .register_type::<DetailedBodyCollision>()
            .add_systems(PreStartup, configure_overload_guard)
            .add_plugins(
                PhysicsPlugins::default()
                    .with_collision_hooks::<topology::SpatialTopologyCollisionHooks>(),
            )
            // Avian's one-vector world gravity is intentionally disabled.
            // Canonical spatial gravity is queried through physics::gravity.
            .insert_resource(Gravity::ZERO)
            .add_plugins((GravityPlugin, CharacterMovementPlugin))
            .add_systems(PreUpdate, chart::prepare_usf_physics_charts)
            .add_systems(
                PostUpdate,
                collision_topology::rebuild_clipped_colliders
                    .in_set(UsfSpatialSet::RuntimeProjection),
            )
            .add_systems(
                PostUpdate,
                update_moved_collider_aabbs::<Collider>
                    .in_set(UsfSpatialSet::BackendRefresh),
            );
    }
}
