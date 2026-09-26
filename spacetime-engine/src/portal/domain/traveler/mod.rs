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

    pub(crate) fn refresh_runtime_cache_after_rebase(&mut self, position: Vec3) {
        self.previous_position = Some(position);
    }

    pub(crate) fn reset_spatial_transition(&mut self, position: Vec3) {
        self.previous_position = Some(position);
    }

}

/// Opts one spatial manifestation into portal-aware partitioning.
///
/// `solver_peer` is a reserved pairwise Avian solver slot, not semantic
/// identity and not an authority partition. While a split is active that slot is
/// temporarily attached as a logical realization of a generic sibling authority
/// partition.
///
/// The generic authority graph is 1..N. This one reserved peer remains contained
/// inside the current pairwise portal/Avian adapter and must not be interpreted
/// as the engine-wide ownership model.
#[derive(Component, Debug)]
pub struct PortalSplitTraveler {
    pub(crate) active: Option<ActivePortalSplit>,
    pub(crate) tick_start: Transform,
    solver_peer: Entity,
}

impl PortalSplitTraveler {
    pub fn new(initial_transform: Transform, solver_peer: Entity) -> Self {
        Self {
            active: None,
            tick_start: initial_transform,
            solver_peer,
        }
    }

    pub fn is_split(&self) -> bool {
        self.active.is_some()
    }

    pub fn solver_peer(&self) -> Entity {
        self.solver_peer
    }

    pub(crate) fn refresh_runtime_cache_after_rebase(&mut self, transform: Transform) {
        self.tick_start = transform;
    }

    pub(crate) fn reset_spatial_transition(&mut self, transform: Transform) {
        self.active = None;
        self.tick_start = transform;
    }

}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ActivePortalSplit {
    /// Portal whose local space currently contains the primary solver realization.
    pub source: Entity,
    /// Peer portal to which the complementary realization is rigidly mapped.
    pub destination: Entity,
    /// Generic authority partition temporarily owning the complementary realization.
    pub partition: Entity,
    /// Concrete backend-local logical realization attached to `partition`.
    pub realization: Entity,
}

/// Runtime bridge that lets one rigid object remain physically coherent while
/// its collision geometry is represented by two portal-linked solver bodies.
///
/// Generic semantic/topological authority is represented by USF authority
/// partitions. This component is only the current pairwise Avian solver adapter:
/// external forces, gravity, mass and inertia stay on the primary solver
/// realization while peer contact deltas are folded back after solving.
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
