//! Generic scale-local capability realization and realized coverage.
//!
//! Demand is intent. Residency is canonical context responsibility.
//! [`UsfCapabilityRealization`] is live capability-local state.
//! [`UsfScaleCoverage`] is the bounded realized fact derived from that state.
//!
//! Coverage is persistent across frames until the underlying realization changes
//! or retires. It is never cleared speculatively before capability planners read
//! it.

use bevy::prelude::*;

use super::{SpatialScale, UsfPosition, UsfSpatialSet};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfScaleRoleMask(u16);

impl UsfScaleRoleMask {
    pub const NONE: Self = Self(0);
    pub const REALIZATION: Self = Self(1 << 0);
    pub const PRESENTATION: Self = Self(1 << 1);
    pub const COLLISION: Self = Self(1 << 2);
    pub const EDITING: Self = Self(1 << 3);

    pub const fn bits(self) -> u16 { self.0 }
    pub const fn is_empty(self) -> bool { self.0 == 0 }
    pub const fn contains(self, role: Self) -> bool { (self.0 & role.0) == role.0 }
    pub const fn union(self, other: Self) -> Self { Self(self.0 | other.0) }
}

/// One live capability-local realization.
///
/// This component belongs on the concrete disposable runtime entity that owns
/// the realization. It records capability state only; it does not own canonical
/// space, residency, semantic identity, or presentation policy.
///
/// `roles == NONE` means the realization entity exists but currently publishes
/// no realized coverage (for example while rebuilding).
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfCapabilityRealization {
    authority: Entity,
    scale: SpatialScale,
    center: UsfPosition,
    half_extent_native: Vec3,
    roles: UsfScaleRoleMask,
    revision: u64,
}

impl UsfCapabilityRealization {
    pub fn new(
        authority: Entity,
        scale: SpatialScale,
        center: UsfPosition,
        half_extent_native: Vec3,
        roles: UsfScaleRoleMask,
        revision: u64,
    ) -> Self {
        Self {
            authority,
            scale,
            center,
            half_extent_native: half_extent_native.abs(),
            roles,
            revision,
        }
    }

    pub const fn authority(self) -> Entity { self.authority }
    pub const fn scale(self) -> SpatialScale { self.scale }
    pub const fn center(self) -> UsfPosition { self.center }
    pub const fn half_extent_native(self) -> Vec3 { self.half_extent_native }
    pub const fn roles(self) -> UsfScaleRoleMask { self.roles }
    pub const fn revision(self) -> u64 { self.revision }

    pub fn set_roles(&mut self, roles: UsfScaleRoleMask) {
        self.roles = roles;
    }

    fn coverage(self, realization: Entity) -> Option<UsfScaleCoverage> {
        (!self.roles.is_empty()).then_some(UsfScaleCoverage {
            realization,
            authority: self.authority,
            scale: self.scale,
            center: self.center,
            half_extent_native: self.half_extent_native,
            roles: self.roles,
            revision: self.revision,
        })
    }
}

/// One bounded realized capability fact.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfScaleCoverage {
    realization: Entity,
    authority: Entity,
    scale: SpatialScale,
    center: UsfPosition,
    half_extent_native: Vec3,
    roles: UsfScaleRoleMask,
    revision: u64,
}

impl UsfScaleCoverage {
    pub const fn realization(self) -> Entity { self.realization }
    pub const fn authority(self) -> Entity { self.authority }
    pub const fn scale(self) -> SpatialScale { self.scale }
    pub const fn center(self) -> UsfPosition { self.center }
    pub const fn half_extent_native(self) -> Vec3 { self.half_extent_native }
    pub const fn roles(self) -> UsfScaleRoleMask { self.roles }
    pub const fn revision(self) -> u64 { self.revision }

    pub fn is_within(
        self,
        point: &UsfPosition,
        required: UsfScaleRoleMask,
        radius_native: f32,
    ) -> bool {
        if !self.roles.contains(required) {
            return false;
        }

        let radius = radius_native.max(0.0);
        let expanded_extent = self.half_extent_native + Vec3::splat(radius);
        let bound = expanded_extent.length() + 1.0;
        let Ok(relative) =
            point.relative_at_scale_bounded(&self.center, self.scale, bound)
        else {
            return false;
        };

        let nearest = Vec3::new(
            relative.x.clamp(-self.half_extent_native.x, self.half_extent_native.x),
            relative.y.clamp(-self.half_extent_native.y, self.half_extent_native.y),
            relative.z.clamp(-self.half_extent_native.z, self.half_extent_native.z),
        );
        (relative - nearest).length_squared() <= radius * radius
    }

    pub fn coarser_scale(self) -> Option<SpatialScale> {
        SpatialScale::new(self.scale.exponent().checked_add(1)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfRefinementAperture {
    authority: Entity,
    coarse_scale: SpatialScale,
    fine_scale: SpatialScale,
    center: UsfPosition,
    half_extent_fine_native: Vec3,
    roles: UsfScaleRoleMask,
}

impl UsfRefinementAperture {
    pub fn from_coverage(coverage: UsfScaleCoverage) -> Option<Self> {
        Some(Self {
            authority: coverage.authority(),
            coarse_scale: coverage.coarser_scale()?,
            fine_scale: coverage.scale(),
            center: coverage.center(),
            half_extent_fine_native: coverage.half_extent_native(),
            roles: coverage.roles(),
        })
    }

    pub const fn authority(self) -> Entity { self.authority }
    pub const fn coarse_scale(self) -> SpatialScale { self.coarse_scale }
    pub const fn fine_scale(self) -> SpatialScale { self.fine_scale }
    pub const fn center(self) -> UsfPosition { self.center }
    pub const fn half_extent_fine_native(self) -> Vec3 { self.half_extent_fine_native }
    pub const fn roles(self) -> UsfScaleRoleMask { self.roles }
}

/// Persistent snapshot of currently realized capability coverage.
///
/// The snapshot is reconciled from live [`UsfCapabilityRealization`] components
/// after capability publication. A realization that disappears automatically
/// disappears from coverage at the next reconciliation.
#[derive(Resource, Debug, Default)]
pub struct UsfScaleCoverageSnapshot {
    revision: u64,
    entries: Vec<UsfScaleCoverage>,
}

impl UsfScaleCoverageSnapshot {
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = UsfScaleCoverage> + '_ {
        self.entries.iter().copied()
    }

    pub fn apertures(
        &self,
        required: UsfScaleRoleMask,
    ) -> impl Iterator<Item = UsfRefinementAperture> + '_ {
        self.entries
            .iter()
            .copied()
            .filter(move |coverage| coverage.roles().contains(required))
            .filter_map(UsfRefinementAperture::from_coverage)
    }

    pub fn has_near(
        &self,
        scale: SpatialScale,
        point: &UsfPosition,
        required: UsfScaleRoleMask,
        radius_native: f32,
    ) -> bool {
        self.entries.iter().copied().any(|coverage| {
            coverage.scale() == scale
                && coverage.is_within(point, required, radius_native)
        })
    }

    pub fn has_near_for_authority(
        &self,
        authority: Entity,
        scale: SpatialScale,
        point: &UsfPosition,
        required: UsfScaleRoleMask,
        radius_native: f32,
    ) -> bool {
        self.entries.iter().copied().any(|coverage| {
            coverage.authority() == authority
                && coverage.scale() == scale
                && coverage.is_within(point, required, radius_native)
        })
    }

    fn reconcile(&mut self, mut next: Vec<UsfScaleCoverage>) {
        next.sort_by_key(|coverage| coverage.realization().to_bits());

        if self.entries != next {
            self.entries = next;
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfCapabilitySet {
    /// Capability subsystems publish/update live realization components here.
    Publish,
    /// Spatial runtime rebuilds persistent coverage from those live facts.
    ReconcileCoverage,
}

fn reconcile_capability_coverage(
    realizations: Query<(Entity, &UsfCapabilityRealization)>,
    mut snapshot: ResMut<UsfScaleCoverageSnapshot>,
) {
    let mut next = Vec::with_capacity(realizations.iter().len());
    for (entity, realization) in &realizations {
        if let Some(coverage) = realization.coverage(entity) {
            next.push(coverage);
        }
    }
    snapshot.reconcile(next);
}

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfScaleCoverageSnapshot>()
        .configure_sets(
            PostUpdate,
            (
                UsfCapabilitySet::Publish,
                UsfCapabilitySet::ReconcileCoverage,
            )
                .chain()
                .before(UsfSpatialSet::SyncSemantic),
        )
        .add_systems(
            PostUpdate,
            reconcile_capability_coverage
                .in_set(UsfCapabilitySet::ReconcileCoverage),
        );
}
