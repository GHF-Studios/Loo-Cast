//! Physical portal endpoints and pair identity.

use bevy::prelude::*;

use super::face::PortalSidedness;

/// Identifies one physical endpoint of a portal pair.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortalEndpoint {
    First,
    Second,
}

impl PortalEndpoint {
    pub(crate) fn other(self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

/// One physical portal aperture.
///
/// The ordinary Bevy [`Transform`] on this entity is authoritative. Rendering
/// may expose one or two directed faces for an aperture, but simulation still
/// treats it as one physical portal plane.
#[derive(Component, Debug, Clone, Copy)]
pub struct Portal {
    pub(crate) endpoint: PortalEndpoint,
    pub destination: Entity,
    pub half_size: Vec2,
    pub sidedness: PortalSidedness,
}

/// The clip-capable world collider currently supporting this endpoint.
///
/// Support is resolved when a portal is placed. Traversal never rediscovers
/// hosts with ad-hoc raycasts; collision topology consumes this stable binding.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(crate) struct PortalSupport(pub Option<Entity>);

/// Whether an endpoint is currently placed in gameplay.
///
/// The entity and its rendering infrastructure persist while inactive. This
/// allows tools to remove/re-place portals without rebuilding the recursive
/// rendering tree or invalidating stable entity references.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PortalActive(pub bool);

impl Default for PortalActive {
    fn default() -> Self {
        Self(false)
    }
}

/// Stable entity IDs of the built-in physical portal pair.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PortalPair {
    pub first: Entity,
    pub second: Entity,
}

impl PortalPair {
    pub fn entity(self, endpoint: PortalEndpoint) -> Entity {
        match endpoint {
            PortalEndpoint::First => self.first,
            PortalEndpoint::Second => self.second,
        }
    }
}

/// Marks the ordinary camera from which portal views are derived.
#[derive(Component)]
pub struct PortalView;
