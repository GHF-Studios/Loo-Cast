//! Ordinary geometric portals.
//!
//! Responsibility split:
//!
//! - `domain`: what a portal/traveler/config *is*;
//! - `topology`: pure geometric mapping/crossing/clipping math;
//! - `simulation`: conventional instantaneous teleportation;
//! - `rendering`: the visual portal illusion.
//!
//! This intentionally stops before USF manifestation-based crossing.

mod domain;
mod rendering;
mod simulation;
mod topology;

#[cfg(test)]
mod tests;

pub use domain::{
    Portal,
    PortalConfig,
    PortalEndpointConfig,
    PortalPair,
    PortalSidedness,
    PortalTraveler,
    PortalVelocity,
    PortalView,
};

pub use rendering::MAIN_PORTAL_LAYER;

use bevy::prelude::*;

use crate::game::SimulationSet;

pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PortalConfig>()
            .add_plugins(
                rendering::
                    PortalRenderingPlugin,
            )
            .add_systems(
                Update,
                simulation::traversal::
                    teleport_travelers
                    .in_set(
                        SimulationSet::Topology,
                    ),
            );
    }
}
