use bevy::prelude::*;

/// Marks an entity whose movement may cross portal apertures.
///
/// The previous position enables segment/plane crossing detection for the
/// conventional unsplit traversal fallback.
#[derive(Component, Debug, Default)]
pub struct PortalTraveler {
    previous_position: Option<Vec3>,
}

impl PortalTraveler {
    pub fn new(position: Vec3) -> Self {
        Self {
            previous_position: Some(position),
        }
    }

    pub(crate) fn previous_position(&self) -> Option<Vec3> {
        self.previous_position
    }

    pub(crate) fn commit_position(&mut self, position: Vec3) {
        self.previous_position = Some(position);
    }
}

/// Opts one authoritative manifestation into the narrow portal-split
/// prototype.
///
/// The peer manifestation is discovered through the generic
/// `UsfManifestationOf` relationship rather than stored here. This component
/// therefore contains only portal-specific orchestration state.
#[derive(Component, Debug)]
pub struct PortalSplitTraveler {
    pub(crate) active: Option<ActivePortalSplit>,
    pub(crate) tick_start: Transform,
}

impl PortalSplitTraveler {
    pub fn new(initial_transform: Transform) -> Self {
        Self {
            active: None,
            tick_start: initial_transform,
        }
    }

    pub fn is_split(&self) -> bool {
        self.active.is_some()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ActivePortalSplit {
    /// Portal whose local space currently contains the authoritative body.
    pub source: Entity,
    /// Peer portal to which the other manifestation is rigidly mapped.
    pub destination: Entity,
}

/// Optional linear velocity transformed alongside a conventional traveler.
#[derive(Component, Debug, Clone, Copy)]
pub struct PortalVelocity(pub Vec3);
