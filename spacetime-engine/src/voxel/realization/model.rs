//! Intent and resolved-demand snapshots across the voxel realization boundary.

use super::*;

/// One capability-specific reason for a voxel materialization scope to exist.
///
/// View and physical demand share geometry/cache machinery without sharing
/// capability authority.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::voxel) struct VoxelRealizationScope {
    pub(super) scope: SpatialDemandScope,
    pub(super) roles: UsfScaleRoleMask,
    pub(super) view_source: Option<Entity>,
    pub(super) residency_half_extent_native: Vec3,
    // Canonical nearest semantic boundary used ONLY for reconstructible work
    // ordering. It never grants capability or semantic authority.
    pub(super) priority_focus: Option<UsfPosition>,
}

impl VoxelRealizationScope {
    pub(in crate::voxel) fn new(scope: SpatialDemandScope, roles: UsfScaleRoleMask) -> Self {
        Self {
            scope,
            roles,
            view_source: None,
            residency_half_extent_native: materialization_residency_extent(
                scope.half_extent_native(),
            ),
            priority_focus: None,
        }
    }

    pub(in crate::voxel) const fn with_view_source(mut self, source: Entity) -> Self {
        self.view_source = Some(source);
        self
    }

    pub(in crate::voxel) const fn with_priority_focus(mut self, focus: UsfPosition) -> Self {
        self.priority_focus = Some(focus);
        self
    }

    pub(in crate::voxel) const fn scope(self) -> SpatialDemandScope {
        self.scope
    }

    pub(in crate::voxel) const fn roles(self) -> UsfScaleRoleMask {
        self.roles
    }

    pub(in crate::voxel) const fn view_source(self) -> Option<Entity> {
        self.view_source
    }

    pub(in crate::voxel) const fn residency_half_extent_native(self) -> Vec3 {
        self.residency_half_extent_native
    }

    pub(in crate::voxel) const fn priority_focus(self) -> Option<UsfPosition> {
        self.priority_focus
    }
}

/// Semantic celestial realization identity. This target can exist before its
/// scale-local `VoxelScaleRealization`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct VoxelRealizationTarget {
    authority: Entity,
    scale: SpatialScale,
}

impl VoxelRealizationTarget {
    pub(in crate::voxel) const fn new(authority: Entity, scale: SpatialScale) -> Self {
        Self { authority, scale }
    }

    pub(in crate::voxel) const fn authority(self) -> Entity {
        self.authority
    }

    pub(in crate::voxel) const fn scale(self) -> SpatialScale {
        self.scale
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum VoxelRealizationIntentTarget {
    ExistingRealization(Entity),
    Celestial(VoxelRealizationTarget),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct VoxelRealizationIntent {
    pub(super) target: VoxelRealizationIntentTarget,
    pub(super) scope: SpatialDemandScope,
    pub(super) roles: UsfScaleRoleMask,
    pub(super) view_source: Option<Entity>,
    pub(super) residency_half_extent_native: Vec3,
    pub(super) priority_focus: Option<UsfPosition>,
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct VoxelRealizationIntentSnapshot {
    intents: Vec<VoxelRealizationIntent>,
}

impl VoxelRealizationIntentSnapshot {
    pub(super) fn iter(&self) -> impl Iterator<Item = &VoxelRealizationIntent> {
        self.intents.iter()
    }

    pub(super) fn push_intent(&mut self, intent: VoxelRealizationIntent) {
        self.intents.push(intent);
    }

    /// Stable publication order keeps authority and source runs contiguous.
    pub(super) fn sort_for_publication(&mut self) {
        self.intents.sort_by_key(|intent| {
            let (kind, owner, scale) = match intent.target {
                VoxelRealizationIntentTarget::ExistingRealization(world) => {
                    (0_u8, world.to_bits(), intent.scope.scale().exponent())
                }
                VoxelRealizationIntentTarget::Celestial(target) => (
                    1_u8,
                    target.authority().to_bits(),
                    target.scale().exponent(),
                ),
            };
            (
                kind,
                owner,
                intent.scope.source().to_bits(),
                Reverse(scale),
                intent.roles.bits(),
                intent.view_source.map(Entity::to_bits).unwrap_or(0),
            )
        });
    }

    pub(super) fn replace_if_changed(&mut self, next: Self) {
        if self.intents != next.intents {
            self.intents = next.intents;
        }
    }

    pub(in crate::voxel) fn celestial_targets(
        &self,
    ) -> impl Iterator<Item = VoxelRealizationTarget> + '_ {
        self.intents
            .iter()
            .filter_map(|intent| match intent.target {
                VoxelRealizationIntentTarget::Celestial(target) => Some(target),
                VoxelRealizationIntentTarget::ExistingRealization(_) => None,
            })
    }

    pub(super) fn push(
        &mut self,
        target: VoxelRealizationIntentTarget,
        scope: SpatialDemandScope,
        roles: UsfScaleRoleMask,
        view_source: Option<Entity>,
        residency_half_extent_native: Vec3,
        priority_focus: Option<UsfPosition>,
    ) {
        self.intents.push(VoxelRealizationIntent {
            target,
            scope,
            roles,
            view_source,
            residency_half_extent_native,
            priority_focus,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct VoxelRealizationDemand {
    pub(super) target_realization: Entity,
    pub(super) scope: SpatialDemandScope,
    pub(super) roles: UsfScaleRoleMask,
    pub(super) view_source: Option<Entity>,
    pub(super) residency_half_extent_native: Vec3,
    pub(super) priority_focus: Option<UsfPosition>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct VoxelDemandSource {
    pub(super) scope: SpatialDemandScope,
    pub(super) minimum_realization_scale: Option<SpatialScale>,
    pub(super) refinement_half_extent_native: Option<Vec3>,
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct VoxelRealizationDemandSnapshot {
    demands: Vec<VoxelRealizationDemand>,
}

impl VoxelRealizationDemandSnapshot {
    pub(super) fn sort_for_publication(&mut self) {
        self.demands.sort_by_key(|demand| {
            (
                demand.target_realization.to_bits(),
                demand.scope.source().to_bits(),
                Reverse(demand.scope.scale().exponent()),
                demand.roles.bits(),
                demand.view_source.map(Entity::to_bits).unwrap_or(0),
            )
        });
    }

    pub(super) fn replace_if_changed(&mut self, next: Self) {
        if self.demands != next.demands {
            self.demands = next.demands;
        }
    }

    pub(in crate::voxel) fn requests_for(
        &self,
        realization: Entity,
    ) -> impl Iterator<Item = VoxelRealizationScope> + '_ {
        // Demands are sorted by target_realization during publication. Binary-search
        // the world's contiguous run instead of rescanning every demand once
        // for every resident VoxelScaleRealization.
        let realization_bits = realization.to_bits();
        let start = self
            .demands
            .partition_point(|demand| demand.target_realization.to_bits() < realization_bits);
        let end = self.demands[start..]
            .partition_point(|demand| demand.target_realization == realization)
            + start;

        self.demands[start..end]
            .iter()
            .map(|demand| VoxelRealizationScope {
                scope: demand.scope,
                roles: demand.roles,
                view_source: demand.view_source,
                residency_half_extent_native: demand.residency_half_extent_native,
                priority_focus: demand.priority_focus,
            })
    }

    pub(super) fn push(
        &mut self,
        target_realization: Entity,
        scope: SpatialDemandScope,
        roles: UsfScaleRoleMask,
        view_source: Option<Entity>,
        residency_half_extent_native: Vec3,
        priority_focus: Option<UsfPosition>,
    ) {
        self.demands.push(VoxelRealizationDemand {
            target_realization,
            scope,
            roles,
            view_source,
            residency_half_extent_native,
            priority_focus,
        });
    }
}
