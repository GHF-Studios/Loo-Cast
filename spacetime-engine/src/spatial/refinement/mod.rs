//! Actual Scale-Slice coverage and adjacent-slice responsibility.
//!
//! Demand is intent. Coverage is realized fact. A finer slice may take
//! responsibility from its adjacent coarser slice only inside bounded regions
//! that have actually been realized by a mechanism.

use bevy::prelude::*;

use super::{SpatialScale, UsfPosition};

mod plan;
pub use plan::{UsfRefinementPlan, UsfRefinementStep};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfScaleCoverage {
    authority: Entity,
    scale: SpatialScale,
    center: UsfPosition,
    half_extent_native: Vec3,
    roles: UsfScaleRoleMask,
}

impl UsfScaleCoverage {
    pub fn new(
        authority: Entity,
        scale: SpatialScale,
        center: UsfPosition,
        half_extent_native: Vec3,
        roles: UsfScaleRoleMask,
    ) -> Self {
        Self {
            authority,
            scale,
            center,
            half_extent_native: half_extent_native.abs(),
            roles,
        }
    }

    pub const fn authority(self) -> Entity { self.authority }
    pub const fn scale(self) -> SpatialScale { self.scale }
    pub const fn center(self) -> UsfPosition { self.center }
    pub const fn half_extent_native(self) -> Vec3 { self.half_extent_native }
    pub const fn roles(self) -> UsfScaleRoleMask { self.roles }

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
        let Ok(relative) = point.relative_at_scale_bounded(&self.center, self.scale, bound) else {
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

#[derive(Resource, Debug, Default)]
pub struct UsfScaleCoverageSnapshot {
    entries: Vec<UsfScaleCoverage>,
}

impl UsfScaleCoverageSnapshot {
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
            coverage.scale() == scale && coverage.is_within(point, required, radius_native)
        })
    }

    /// Tests realized coverage from one semantic authority only.
    ///
    /// Physical handoff must never be satisfied by an unrelated mechanism that
    /// merely happens to publish coverage at the same Scale Slice nearby.
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

    pub(crate) fn publish(&mut self, coverage: UsfScaleCoverage) {
        self.entries.push(coverage);
    }

    fn clear(&mut self) { self.entries.clear(); }
}

pub(in crate::spatial) fn clear_scale_coverage(
    mut coverage: ResMut<UsfScaleCoverageSnapshot>,
) {
    coverage.clear();
}
