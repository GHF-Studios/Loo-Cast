//! Live capability facts and bounded realized coverage records.

use super::*;

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

    pub const fn authority(self) -> Entity {
        self.authority
    }
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
    pub const fn center(self) -> UsfPosition {
        self.center
    }
    pub const fn half_extent_native(self) -> Vec3 {
        self.half_extent_native
    }
    pub const fn roles(self) -> UsfScaleRoleMask {
        self.roles
    }
    pub const fn revision(self) -> u64 {
        self.revision
    }

    pub fn set_roles(&mut self, roles: UsfScaleRoleMask) {
        self.roles = roles;
    }

    pub(super) fn coverage(self, realization: Entity) -> Option<UsfScaleCoverage> {
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

/// One bounded capability fact published through a batched producer.
///
/// Some capabilities own thousands of sparse realized cells without owning an
/// ECS object per cell. The producer entity is lifecycle identity only; it does
/// not become semantic authority or presentation identity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCapabilityCoverageRecord {
    authority: Entity,
    scale: SpatialScale,
    center: UsfPosition,
    half_extent_native: Vec3,
    roles: UsfScaleRoleMask,
    revision: u64,
}

impl UsfCapabilityCoverageRecord {
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

    pub(super) fn coverage(self, producer: Entity) -> Option<UsfScaleCoverage> {
        (!self.roles.is_empty()).then_some(UsfScaleCoverage {
            realization: producer,
            authority: self.authority,
            scale: self.scale,
            center: self.center,
            half_extent_native: self.half_extent_native,
            roles: self.roles,
            revision: self.revision,
        })
    }
}

/// Store-backed capability facts owned by one ECS producer.
///
/// This is deliberately a batch rather than one entity per realized cell. Batch
/// order must be deterministic for stable snapshot equality.
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct UsfCapabilityCoverageBatch {
    records: Vec<UsfCapabilityCoverageRecord>,
}

impl UsfCapabilityCoverageBatch {
    pub fn new(records: Vec<UsfCapabilityCoverageRecord>) -> Self {
        Self { records }
    }

    pub fn records(&self) -> &[UsfCapabilityCoverageRecord] {
        &self.records
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
    pub const fn realization(self) -> Entity {
        self.realization
    }
    pub const fn authority(self) -> Entity {
        self.authority
    }
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
    pub const fn center(self) -> UsfPosition {
        self.center
    }
    pub const fn half_extent_native(self) -> Vec3 {
        self.half_extent_native
    }
    pub const fn roles(self) -> UsfScaleRoleMask {
        self.roles
    }
    pub const fn revision(self) -> u64 {
        self.revision
    }

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
            relative
                .x
                .clamp(-self.half_extent_native.x, self.half_extent_native.x),
            relative
                .y
                .clamp(-self.half_extent_native.y, self.half_extent_native.y),
            relative
                .z
                .clamp(-self.half_extent_native.z, self.half_extent_native.z),
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

    pub const fn authority(self) -> Entity {
        self.authority
    }
    pub const fn coarse_scale(self) -> SpatialScale {
        self.coarse_scale
    }
    pub const fn fine_scale(self) -> SpatialScale {
        self.fine_scale
    }
    pub const fn center(self) -> UsfPosition {
        self.center
    }
    pub const fn half_extent_fine_native(self) -> Vec3 {
        self.half_extent_fine_native
    }
    pub const fn roles(self) -> UsfScaleRoleMask {
        self.roles
    }
}
