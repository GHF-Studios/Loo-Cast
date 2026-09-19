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

    pub(crate) fn rebase_local_origin(&mut self, shift: Vec3) {
        if let Some(previous) = &mut self.previous_position {
            *previous -= shift;
        }
    }

    pub(crate) fn reset_spatial_transition(&mut self, position: Vec3) {
        self.previous_position = Some(position);
    }

}

/// Opts one spatial manifestation into portal-aware partitioning.
///
/// `peer` is the reserved logical/physics projection for the opposite side of
/// the active portal. Presentation projections are associated independently, so
/// semantic entities can have any number of other manifestations without
/// confusing topology peers with presentation copies.
#[derive(Component, Debug)]
pub struct PortalSplitTraveler {
    pub(crate) active: Option<ActivePortalSplit>,
    pub(crate) tick_start: Transform,
    peer: Entity,
}

impl PortalSplitTraveler {
    pub fn new(initial_transform: Transform, peer: Entity) -> Self {
        Self {
            active: None,
            tick_start: initial_transform,
            peer,
        }
    }

    pub fn is_split(&self) -> bool {
        self.active.is_some()
    }

    pub fn peer(&self) -> Entity {
        self.peer
    }

    pub(crate) fn rebase_local_origin(&mut self, shift: Vec3) {
        self.tick_start.translation -= shift;
    }

    pub(crate) fn reset_spatial_transition(&mut self, transform: Transform) {
        self.active = None;
        self.tick_start = transform;
    }

}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ActivePortalSplit {
    /// Portal whose local space currently contains the authoritative body.
    pub source: Entity,
    /// Peer portal to which the other manifestation is rigidly mapped.
    pub destination: Entity,
}

/// Runtime bridge that lets a dynamic rigid body remain one physical object
/// while its collision geometry is represented by two portal-linked solver bodies.
///
/// External forces, gravity, mass and inertia belong to the authoritative body.
/// The peer receives only mapped baseline velocity plus solver contacts; the
/// resulting contact delta is folded back into the authority after Avian solves.
#[derive(Component, Debug, Clone, Copy)]
pub struct PortalRigidSplitBody {
    pub(crate) peer_baseline_linear: Vec3,
    pub(crate) peer_baseline_angular: Vec3,
    pub(crate) peer_solver_active: bool,
}

impl Default for PortalRigidSplitBody {
    fn default() -> Self {
        Self {
            peer_baseline_linear: Vec3::ZERO,
            peer_baseline_angular: Vec3::ZERO,
            peer_solver_active: false,
        }
    }
}

/// Optional linear velocity transformed alongside a conventional traveler.
#[derive(Component, Debug, Clone, Copy)]
pub struct PortalVelocity(pub Vec3);
