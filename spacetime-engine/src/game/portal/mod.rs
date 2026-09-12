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

mod domain;
mod rendering;
mod simulation;
mod topology;

#[cfg(test)]
mod tests;

pub use domain::{
    Portal, PortalActive, PortalCommand, PortalConfig, PortalEndpoint, PortalEndpointConfig,
    PortalPair, PortalSidedness, PortalSplitTraveler, PortalTraveler, PortalVelocity, PortalView,
};

pub use rendering::MAIN_PORTAL_LAYER;

use bevy::prelude::*;

use crate::{game::SimulationSet, physics::character::CharacterMovementSet};

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
            );
    }
}
