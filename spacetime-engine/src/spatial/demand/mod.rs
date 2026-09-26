//! Generic bounded spatial interest.
//!
//! A [`SpatialDemandSource`] answers only: "what canonical region around this
//! manifestation currently matters, and with what priority?" It does not choose
//! representation scales, generate a multi-scale realization plan, or imply that
//! any particular subsystem must materialize data.
//!
//! Capability planners consume the resulting [`SpatialDemandSnapshot`] and make
//! their own realization decisions. [`SpatialRefinementDemand`] is a separate
//! detail requirement consumed by those planners; it deliberately does not
//! manufacture extra generic demand scopes.

use bevy::prelude::*;

use super::{SpatialScale, UsfPosition, UsfRefinementPlan, UsfScaleLayer, UsfSpatialFrame};

/// One bounded source of generic spatial interest.
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

/// Requested additional spatial detail for capability-specific realization.
///
/// This is *not* another generic interest volume. [`UsfRefinementPlan`]
/// turns this requested tip into the current-relative ancestor branch over the
/// Scale Slices supported by a capability. The capability still decides what it
/// realizes inside each planned scope; it does not redefine the taper itself.
///
/// `None` means "no additional refinement request". It must not mean "suppress
/// the capability's ordinary realization for the generic spatial-interest
/// scope".
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

/// One canonical bounded interest scope expressed in one runtime Scale Slice.
///
/// The scale records the source's current numerical/interaction chart. It does
/// not imply that all coarser/finer representations should exist.
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
