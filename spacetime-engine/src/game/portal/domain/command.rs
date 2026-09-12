//! Public mutation commands for the persistent portal pair.
//!
//! Gameplay tools emit commands instead of reaching into portal entities. The
//! portal simulation layer owns activation, transforms and presentation state.

use bevy::prelude::*;

use super::portal::PortalEndpoint;

/// Requested change to one or both persistent portal endpoints.
#[derive(Message, Debug, Clone)]
pub enum PortalCommand {
    /// Place or replace an endpoint at an authoritative rigid world transform.
    /// Scale must remain [`Vec3::ONE`].
    Place {
        endpoint: PortalEndpoint,
        transform: Transform,
    },
    /// Remove one endpoint while preserving its entity/infrastructure.
    Remove {
        endpoint: PortalEndpoint,
    },
    /// Remove both endpoints.
    RemovePair,
}
