use bevy::prelude::*;

/// Identifies one endpoint of the built-in pair.
///
/// Keeping this explicit makes recursive render paths human-readable instead of
/// encoding them as bit arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PortalEndpoint {
    First,
    Second,
}

impl PortalEndpoint {
    pub(crate) const ALL: [Self; 2] = [
        Self::First,
        Self::Second,
    ];

    pub(crate) fn other(self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::First,
        }
    }
}

/// Which geometric side of an oriented portal plane is involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PortalSide {
    Front,
    Back,
}

/// Determines which physical sides of an aperture are active.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
)]
pub enum PortalSidedness {
    /// Only the portal's local +Z side is active.
    OneSided,

    /// Both local +Z and local -Z sides are active.
    #[default]
    TwoSided,
}

impl PortalSidedness {
    pub(crate) fn allows(self, side: PortalSide) -> bool {
        match self {
            Self::OneSided => side == PortalSide::Front,
            Self::TwoSided => true,
        }
    }

    pub(crate) fn is_two_sided(self) -> bool {
        matches!(self, Self::TwoSided)
    }
}

/// One physical portal aperture.
///
/// The ordinary Bevy [`Transform`] on the same entity is authoritative.
/// Translation and arbitrary 3D rotation may be changed at runtime.
///
/// Unit scale is intentionally required: aperture dimensions live in
/// [`half_size`](Self::half_size) instead of transform scale.
#[derive(Component, Debug, Clone, Copy)]
pub struct Portal {
    pub(crate) endpoint: PortalEndpoint,
    pub destination: Entity,
    pub half_size: Vec2,
    pub sidedness: PortalSidedness,
}

/// Entity IDs of the built-in pair.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PortalPair {
    pub first: Entity,
    pub second: Entity,
}

impl PortalPair {
    pub(crate) fn entity(self, endpoint: PortalEndpoint) -> Entity {
        match endpoint {
            PortalEndpoint::First => self.first,
            PortalEndpoint::Second => self.second,
        }
    }
}

/// Marks the ordinary camera from which portal views are derived.
#[derive(Component)]
pub struct PortalView;
