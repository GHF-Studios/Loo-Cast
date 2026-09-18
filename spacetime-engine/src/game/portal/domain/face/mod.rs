//! Directed portal faces.
//!
//! A physical portal plane may expose one or two directed faces.
//!
//! Each face is treated by rendering as an ordinary one-sided portal. This is
//! deliberately different from making one mesh/material geometrically
//! double-sided.

use super::portal::PortalEndpoint;

/// One geometric side of a physical portal plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PortalSide {
    /// Local +Z side.
    Front,

    /// Local -Z side.
    Back,
}

impl PortalSide {
    pub(crate) fn opposite(self) -> Self {
        match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
        }
    }
}

/// One directed view through a physical portal.
///
/// A two-sided physical portal therefore contributes two independent
/// [`PortalFace`]s to the render tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct PortalFace {
    pub(crate) endpoint: PortalEndpoint,
    pub(crate) side: PortalSide,
}

impl PortalFace {
    /// Stable branch ordering for recursive rendering.
    pub(crate) const ALL: [Self; 4] = [
        Self {
            endpoint: PortalEndpoint::First,
            side: PortalSide::Front,
        },
        Self {
            endpoint: PortalEndpoint::First,
            side: PortalSide::Back,
        },
        Self {
            endpoint: PortalEndpoint::Second,
            side: PortalSide::Front,
        },
        Self {
            endpoint: PortalEndpoint::Second,
            side: PortalSide::Back,
        },
    ];

    pub(crate) fn branch_index(self) -> usize {
        match (self.endpoint, self.side) {
            (PortalEndpoint::First, PortalSide::Front) => 0,

            (PortalEndpoint::First, PortalSide::Back) => 1,

            (PortalEndpoint::Second, PortalSide::Front) => 2,

            (PortalEndpoint::Second, PortalSide::Back) => 3,
        }
    }

    /// Surface facing the virtual camera immediately after traversing this
    /// face.
    ///
    /// Example:
    ///
    /// - enter A from A-front;
    /// - virtual camera appears on B's physical back side;
    /// - B-back would face that camera and therefore has to be omitted.
    pub(crate) fn exit_face(self) -> Self {
        Self {
            endpoint: self.endpoint.other(),
            side: self.side.opposite(),
        }
    }
}

/// Determines which directed faces a physical portal exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PortalSidedness {
    /// Only local +Z/front is active.
    OneSided,

    /// Front and back are independently active.
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
}
