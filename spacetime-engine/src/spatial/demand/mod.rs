//! Hierarchical multi-scale spatial demand.
//!
//! One source produces a sparse vertical spine of realization demand from
//! the source manifestation's own Scale Slice. Presentation/view scale is not
//! realization authority.

use bevy::prelude::*;

use super::{
    SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfScaleLayer, UsfSpatialFrame,
};

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SpatialDemandSource {
    half_extent_native: Vec3,
    priority: i32,
    enabled: bool,
}

impl SpatialDemandSource {
    pub fn cuboid(half_extent_native: Vec3) -> Self {
        Self {
            half_extent_native: Vec3::new(
                sanitize_extent(half_extent_native.x),
                sanitize_extent(half_extent_native.y),
                sanitize_extent(half_extent_native.z),
            ),
            priority: 0,
            enabled: true,
        }
    }

    pub const fn half_extent_native(&self) -> Vec3 {
        self.half_extent_native
    }
    pub const fn priority(&self) -> i32 {
        self.priority
    }
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }
}

/// Optional bounded refinement spine beneath a spatial demand source.
///
/// The source's ordinary extent continues to propagate to coarser Scale Slices.
/// Finer slices instead receive this bounded scale-local aperture, preventing a
/// coarse physical window from exploding into an enormous fine realization.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SpatialRefinementDemand {
    minimum_scale: Option<SpatialScale>,
    half_extent_native: Vec3,
}

impl SpatialRefinementDemand {
    pub fn cuboid(half_extent_native: Vec3) -> Self {
        Self {
            minimum_scale: None,
            half_extent_native: Vec3::new(
                sanitize_extent(half_extent_native.x),
                sanitize_extent(half_extent_native.y),
                sanitize_extent(half_extent_native.z),
            ),
        }
    }

    pub const fn minimum_scale(&self) -> Option<SpatialScale> {
        self.minimum_scale
    }

    pub const fn half_extent_native(&self) -> Vec3 {
        self.half_extent_native
    }

    pub fn request_through(&mut self, scale: SpatialScale) {
        self.minimum_scale = Some(scale);
    }

    pub fn clear(&mut self) {
        self.minimum_scale = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialDemandScope {
    source: Entity,
    scale: SpatialScale,
    center: UsfPosition,
    half_extent_native: Vec3,
    priority: i32,
}

impl SpatialDemandScope {
    #[cfg(test)]
    pub(crate) const fn new(
        source: Entity,
        center: UsfPosition,
        half_extent_native: Vec3,
        priority: i32,
    ) -> Self {
        Self::at_scale(
            source,
            SpatialScale::ZERO,
            center,
            half_extent_native,
            priority,
        )
    }

    pub(crate) const fn at_scale(
        source: Entity,
        scale: SpatialScale,
        center: UsfPosition,
        half_extent_native: Vec3,
        priority: i32,
    ) -> Self {
        Self {
            source,
            scale,
            center,
            half_extent_native,
            priority,
        }
    }

    pub const fn source(self) -> Entity {
        self.source
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
    pub const fn priority(self) -> i32 {
        self.priority
    }
}

#[derive(Resource, Debug, Default)]
pub struct SpatialDemandSnapshot {
    scopes: Vec<SpatialDemandScope>,
}

impl SpatialDemandSnapshot {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = SpatialDemandScope> + '_ {
        self.scopes.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.scopes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialDemandSet {
    Collect,
}

fn sanitize_extent(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

mod systems;

pub(super) use systems::configure;

#[cfg(test)]
mod tests;
