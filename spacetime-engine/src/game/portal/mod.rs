//! Ordinary geometric portals.
//!
//! Responsibility split:
//!
//! - `domain`: persistent portal data and the public [`PortalCommand`] protocol;
//! - `topology`: pure mapping/crossing/clipping math;
//! - `simulation`: applies commands and owns runtime traversal/splitting;
//! - `rendering`: the recursive visual portal illusion.
//!
//! Gameplay and mods should normally mutate portals by sending
//! [`PortalCommand`] rather than editing portal entities directly. Rendering
//! details remain private to this module.

pub(crate) mod devtools;
mod domain;
mod rendering;
mod simulation;
mod topology;

#[cfg(test)]
mod tests;

pub use domain::{
    Portal, PortalActive, PortalCommand, PortalConfig, PortalEndpoint, PortalEndpointConfig,
    PortalPair, PortalRigidSplitBody, PortalSidedness, PortalSplitTraveler, PortalTraveler,
    PortalVelocity, PortalView,
};

pub use rendering::{DERIVED_VIEW_LAYER, MAIN_PORTAL_LAYER, PortalSplitVisual};

pub(crate) use topology::{
    crossing::crossed_aperture_fraction, mapping::map_transform as map_through_portal,
};

use avian3d::{
    dynamics::{
        integrator::IntegrationSystems,
        solver::schedule::{SolverSystems, SubstepSchedule, SubstepSolverSystems},
    },
    prelude::PhysicsSystems,
    schedule::PhysicsSchedule,
};
use bevy::prelude::*;

use crate::{
    game::SimulationSet,
    physics::character::CharacterMovementSet,
    spatial::{UsfOriginRebased, UsfSpatialSet},
};

pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        use simulation::split::PortalSplitSet;

        app.init_resource::<PortalConfig>()
            .register_type::<PortalActive>()
            .add_message::<PortalCommand>()
            .add_plugins(rendering::PortalRenderingPlugin)
            .configure_sets(
                FixedUpdate,
                PortalSplitSet::Prepare.before(CharacterMovementSet::Simulate),
            )
            .configure_sets(
                FixedUpdate,
                PortalSplitSet::MaterializeBeforeMotor
                    .after(PortalSplitSet::Prepare)
                    .before(CharacterMovementSet::Simulate),
            )
            .configure_sets(
                FixedUpdate,
                PortalSplitSet::Resolve.after(CharacterMovementSet::Simulate),
            )
            .configure_sets(
                FixedUpdate,
                PortalSplitSet::MaterializeAfterMotor.after(PortalSplitSet::Resolve),
            )
            .add_systems(
                Update,
                (
                    simulation::control::apply_portal_commands,
                    simulation::traversal::teleport_travelers,
                )
                    .chain()
                    .in_set(SimulationSet::Topology),
            )
            .add_systems(
                FixedUpdate,
                (
                    simulation::split::prepare_portal_splits.in_set(PortalSplitSet::Prepare),
                    simulation::split::materialize_portal_splits
                        .in_set(PortalSplitSet::MaterializeBeforeMotor),
                    simulation::split::resolve_portal_splits.in_set(PortalSplitSet::Resolve),
                    simulation::split::materialize_portal_splits
                        .in_set(PortalSplitSet::MaterializeAfterMotor),
                ),
            )
            .add_systems(
                FixedPostUpdate,
                simulation::rigid_split::apply_peer_character_pushes
                    .in_set(CharacterMovementSet::PushDynamics)
                    .in_set(PhysicsSystems::First),
            )
            .add_systems(
                FixedPostUpdate,
                simulation::rigid_split::prepare_rigid_splits
                    .after(CharacterMovementSet::PushDynamics)
                    .before(PhysicsSystems::Prepare),
            )
            .add_systems(
                SubstepSchedule,
                simulation::rigid_split::sync_rigid_split_solver_peers
                    .after(IntegrationSystems::Velocity)
                    .before(SubstepSolverSystems::WarmStart),
            )
            .add_systems(
                SubstepSchedule,
                simulation::rigid_split::couple_rigid_split_solver_peers
                    .after(SubstepSolverSystems::SolveConstraints)
                    .before(IntegrationSystems::Position),
            )
            .add_systems(
                SubstepSchedule,
                simulation::rigid_split::couple_rigid_split_solver_peers
                    .after(SubstepSolverSystems::Damping),
            )
            .add_systems(
                PhysicsSchedule,
                simulation::rigid_split::couple_rigid_split_solver_peers
                    .after(SolverSystems::Restitution)
                    .before(SolverSystems::Finalize),
            )
            .add_systems(
                FixedPostUpdate,
                (
                    simulation::rigid_split::receive_peer_dynamic_contact_pushes,
                    simulation::rigid_split::reconcile_rigid_splits,
                )
                    .chain()
                    .after(CharacterMovementSet::ReceiveDynamics),
            )
            .add_systems(
                PostUpdate,
                rebase_portal_local_caches.after(UsfSpatialSet::Rebase),
            );
    }
}

fn rebase_portal_local_caches(
    mut rebases: MessageReader<UsfOriginRebased>,
    mut travelers: Query<&mut PortalTraveler>,
    mut split_travelers: Query<&mut PortalSplitTraveler>,
) {
    let shift = rebases
        .read()
        .fold(Vec3::ZERO, |total, rebase| total + rebase.local_shift);
    if shift == Vec3::ZERO {
        return;
    }

    for mut traveler in &mut travelers {
        traveler.rebase_local_origin(shift);
    }
    for mut traveler in &mut split_travelers {
        traveler.rebase_local_origin(shift);
    }
}
