//! Ordinary geometric portals.
//!
//! Responsibility split:
//!
//! - `domain`: persistent portal data and the public [`PortalCommand`] protocol;
//! - `topology`: pure mapping/crossing/clipping math;
//! - `simulation`: applies commands and conventional instantaneous traversal;
//! - `rendering`: the recursive visual portal illusion.
//!
//! Gameplay and mods should normally mutate portals by sending
//! [`PortalCommand`] rather than editing portal entities directly. Rendering
//! details remain private to this module. This intentionally stops before USF
//! manifestation-based crossing.

mod domain;
mod rendering;
mod simulation;
mod topology;

#[cfg(test)]
mod tests;

pub use domain::{
    Portal,
    PortalActive,
    PortalCommand,
    PortalConfig,
    PortalEndpoint,
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
            .register_type::<PortalActive>()
            .add_message::<PortalCommand>()
            .add_plugins(rendering::PortalRenderingPlugin)
            .add_systems(
                Update,
                (
                    simulation::control::apply_portal_commands,
                    simulation::traversal::teleport_travelers,
                )
                    .chain()
                    .in_set(SimulationSet::Topology),
            );
    }
}
