use bevy::prelude::*;

use super::face::{
    PortalEndpoint,
    PortalSidedness,
};

/// One physical portal aperture.
///
/// The ordinary Bevy [`Transform`] on this entity is authoritative.
///
/// Rendering may expose one or two directed [`PortalFace`](super::PortalFace)s
/// for this physical aperture, but simulation still treats this as one portal
/// plane.
#[derive(Component, Debug, Clone, Copy)]
pub struct Portal {
    pub(crate) endpoint: PortalEndpoint,
    pub destination: Entity,
    pub half_size: Vec2,
    pub sidedness: PortalSidedness,
}

/// Entity IDs of the built-in portal pair.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PortalPair {
    pub first: Entity,
    pub second: Entity,
}

impl PortalPair {
    pub(crate) fn entity(
        self,
        endpoint: PortalEndpoint,
    ) -> Entity {
        match endpoint {
            PortalEndpoint::First => {
                self.first
            }

            PortalEndpoint::Second => {
                self.second
            }
        }
    }
}

/// Marks the ordinary camera from which portal views are derived.
#[derive(Component)]
pub struct PortalView;