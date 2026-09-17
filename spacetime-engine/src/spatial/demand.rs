//! Generic canonical spatial-demand sources.
//!
//! A demand source states that some bounded semantic neighborhood is currently
//! interesting. It does not choose which representation is built there and it
//! owns no loaded/materialized state. Individual realization subsystems opt in
//! separately and merge these scopes according to their own addressing/policy.

use bevy::prelude::*;

use super::{UsfPosition, UsfPositionError, UsfSpatialFrame};

/// A bounded request for spatial realization around this entity's runtime
/// manifestation.
///
/// The local [`GlobalTransform`] is projected through [`UsfSpatialFrame`] each
/// frame into canonical space. `half_extent_native` is deliberately bounded
/// local shape data in units native to the current leaf scale; it is not a
/// universe-wide coordinate and it is not tied to voxel chunk dimensions.
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

/// One frame's canonical interpretation of an enabled demand source.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialDemandScope {
    source: Entity,
    center: UsfPosition,
    half_extent_native: Vec3,
    priority: i32,
}

impl SpatialDemandScope {
    pub(crate) const fn new(
        source: Entity,
        center: UsfPosition,
        half_extent_native: Vec3,
        priority: i32,
    ) -> Self {
        Self {
            source,
            center,
            half_extent_native,
            priority,
        }
    }

    pub const fn source(self) -> Entity {
        self.source
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

/// Ephemeral canonical demand collected for the current frame.
///
/// This resource intentionally contains requests only. It is not a registry of
/// materialized state and does not merge representation-specific addresses.
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

pub(super) fn configure(app: &mut App) {
    app.init_resource::<SpatialDemandSnapshot>()
        .configure_sets(Update, SpatialDemandSet::Collect)
        .add_systems(Update, collect_spatial_demand.in_set(SpatialDemandSet::Collect));
}

fn collect_spatial_demand(
    frame: Res<UsfSpatialFrame>,
    sources: Query<(Entity, &GlobalTransform, &SpatialDemandSource)>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
) {
    snapshot.scopes.clear();

    for (entity, transform, source) in &sources {
        if !source.enabled() {
            continue;
        }

        let local = transform.translation();
        let Ok(center) = canonical_center(*frame.origin(), local) else {
            error!(
                ?entity,
                ?local,
                "spatial demand source could not project into canonical USF space"
            );
            continue;
        };

        snapshot.scopes.push(SpatialDemandScope::new(
            entity,
            center,
            source.half_extent_native(),
            source.priority(),
        ));
    }
}

fn canonical_center(
    frame_origin: UsfPosition,
    local_translation: Vec3,
) -> Result<UsfPosition, UsfPositionError> {
    frame_origin.translated_native(local_translation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_extent_is_bounded_and_toggleable() {
        let mut source = SpatialDemandSource::cuboid(Vec3::new(-5.0, 12.0, 20.0)).with_priority(7);
        assert_eq!(source.half_extent_native(), Vec3::new(0.0, 12.0, 20.0));
        assert_eq!(source.priority(), 7);
        assert!(source.enabled());
        assert!(!source.toggle());
        assert!(!source.enabled());
    }

    #[test]
    fn canonical_center_is_invariant_under_equivalent_rebase() {
        let origin = UsfPosition::default();
        let local = Vec3::new(341.25, -97.0, 18.5);
        let shift = Vec3::new(256.0, 0.0, 0.0);
        let before = canonical_center(origin, local).unwrap();
        let rebased_origin = origin.translated_native(shift).unwrap();
        let after = canonical_center(rebased_origin, local - shift).unwrap();

        assert_eq!(before, after);
    }
}
