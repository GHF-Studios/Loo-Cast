//! Physics integration owned by Spacetime Engine.
//!
//! ## Integration
//!
//! Avian supplies rigid-body infrastructure. Engine character, gravity, collision-query, and
//! topology modules adapt it to canonical USF space; chart rebases refresh its bounded runtime
//! projection.
//!
//! ## Module map
//!
//! - `character`: Kinematic character movement.
//! - `chart_rebase`: Coherent floating-origin rebasing for the Avian backend.
//! - `collision_query`: Canonical swept-collision query boundary.
//! - `collision_topology`: Derived collision-space realization.
//! - `gravity`: Canonical USF gravity.
//! - `hull`: Canonical detailed physical box-hull description.
//! - `interaction_handoff`: Destination-space admission for physical interaction-chart handoff.
//! - `slice`: Scale Slice partition integration for the Avian physics backend.
//! - `topology`: Reusable spatial-topology primitives.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

pub mod character;
mod chart_rebase;
pub mod collision_query;
pub mod collision_topology;
pub mod gravity;
mod hull;
mod interaction_handoff;
pub mod slice;
pub mod topology;

pub use hull::{DetailedBodyCollision, PhysicalBoxHull};

use std::time::Duration;

use avian3d::{
    collider_tree::update_moved_collider_aabbs,
    prelude::{Collider, Gravity, PhysicsPlugins},
};
use bevy::{prelude::*, time::Virtual};

use crate::spatial::{UsfInteractionHandoffSet, UsfSpatialSet};

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
        collision_query::configure(app);

        app.register_type::<PhysicalBoxHull>()
            .register_type::<DetailedBodyCollision>()
            .add_systems(PreStartup, configure_overload_guard)
            .add_plugins(
                PhysicsPlugins::default()
                    .with_collision_hooks::<topology::SpatialTopologyCollisionHooks>(),
            )
            // Keep Avian tree maintenance asynchronous so BVH work may
            // overlap unrelated CPU work instead of extending PhysicsSchedule's
            // serial critical path. If Tracy shows EndOptimize waiting again,
            // the durable fix is dedicated physics-maintenance capacity.
            // Avian's one-vector world gravity is intentionally disabled.
            // Canonical spatial gravity is queried through physics::gravity.
            .insert_resource(Gravity::ZERO)
            .add_plugins((GravityPlugin, CharacterMovementPlugin))
            .add_systems(PreUpdate, slice::sync_usf_physics_slice_metadata)
            .add_systems(
                PostUpdate,
                interaction_handoff::guard_coarsening_interaction_handoffs
                    .in_set(UsfInteractionHandoffSet::Providers),
            )
            .add_systems(
                PostUpdate,
                collision_topology::rebuild_clipped_colliders
                    .in_set(UsfSpatialSet::RuntimeProjection),
            )
            .add_systems(
                PostUpdate,
                (
                    chart_rebase::apply_usf_rebases_to_avian,
                    update_moved_collider_aabbs::<Collider>,
                )
                    .chain()
                    .in_set(UsfSpatialSet::BackendRefresh),
            );
    }
}
