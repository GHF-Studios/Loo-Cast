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

use std::collections::HashMap;

use bevy::{math::DVec3, prelude::*};

use super::{SpatialScale, UsfPosition, UsfScaleLayer, UsfRuntimeChartState};

/// One bounded source of generic spatial interest.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SpatialDemandSource {
    half_extent_native: Vec3,
    // Optional physical authority for local working-set size. When present,
    // native extent is derived at the actual current/transition Scale.
    half_extent_metres: Option<Vec3>,
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
            half_extent_metres: None,
            priority: 0,
            enabled: true,
        }
    }

    pub fn cuboid_metres(half_extent_metres: Vec3) -> Self {
        let half_extent_metres = Vec3::new(
            sanitize_extent(half_extent_metres.x),
            sanitize_extent(half_extent_metres.y),
            sanitize_extent(half_extent_metres.z),
        );
        Self {
            // Retained as a readable fallback/debug value. Metric-authored
            // runtime collection uses half_extent_native_at().
            half_extent_native: half_extent_metres,
            half_extent_metres: Some(half_extent_metres),
            priority: 0,
            enabled: true,
        }
    }

    pub const fn half_extent_native(&self) -> Vec3 {
        self.half_extent_native
    }

    pub fn half_extent_native_at(&self, scale: SpatialScale) -> Vec3 {
        self.half_extent_metres.map_or(self.half_extent_native, |metres| {
            Vec3::new(
                scale.metres_to_native_f32(metres.x),
                scale.metres_to_native_f32(metres.y),
                scale.metres_to_native_f32(metres.z),
            )
        })
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
/// `None` means the current/source Scale Slice is the finest requested tip.
/// Capability refinement still includes every supported coarser ancestor.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SpatialRefinementDemand {
    minimum_scale: Option<SpatialScale>,
    half_extent_native: Vec3,
    // Physical tip footprint. It must be converted at the refinement TIP,
    // not necessarily at the source's current Scale.
    half_extent_metres: Option<Vec3>,
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
            half_extent_metres: None,
        }
    }

    pub fn cuboid_metres(half_extent_metres: Vec3) -> Self {
        let half_extent_metres = Vec3::new(
            sanitize_extent(half_extent_metres.x),
            sanitize_extent(half_extent_metres.y),
            sanitize_extent(half_extent_metres.z),
        );
        Self {
            minimum_scale: None,
            half_extent_native: half_extent_metres,
            half_extent_metres: Some(half_extent_metres),
        }
    }

    pub const fn minimum_scale(&self) -> Option<SpatialScale> {
        self.minimum_scale
    }

    pub const fn half_extent_native(&self) -> Vec3 {
        self.half_extent_native
    }

    pub fn half_extent_native_at(&self, scale: SpatialScale) -> Vec3 {
        self.half_extent_metres.map_or(self.half_extent_native, |metres| {
            Vec3::new(
                scale.metres_to_native_f32(metres.x),
                scale.metres_to_native_f32(metres.y),
                scale.metres_to_native_f32(metres.z),
            )
        })
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

/// Canonical motion hints for active generic spatial-demand sources.
///
/// Motion is deliberately separate from [`SpatialDemandScope`] so continuous
/// velocity changes do not churn demand topology or force unrelated capability
/// planners to rebuild their regions every physics tick.
#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub struct SpatialDemandMotionSnapshot {
    velocities_metres_per_second: HashMap<Entity, DVec3>,
}

impl SpatialDemandMotionSnapshot {
    pub fn velocity_metres_per_second(&self, source: Entity) -> DVec3 {
        self.velocities_metres_per_second
            .get(&source)
            .copied()
            .unwrap_or(DVec3::ZERO)
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
