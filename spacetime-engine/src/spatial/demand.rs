//! Generic spatial demand interpreted through USF simulation layers.

use bevy::prelude::*;

use super::{
    SpatialScale, UsfActiveScaleLayer, UsfPosition, UsfScaleLayer, UsfScaleLayerFrames,
    UsfViewFrame,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialDemandScope {
    source: Entity,
    scale: SpatialScale,
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

pub(super) fn configure(app: &mut App) {
    app.init_resource::<SpatialDemandSnapshot>()
        .configure_sets(Update, SpatialDemandSet::Collect)
        .add_systems(
            Update,
            collect_spatial_demand.in_set(SpatialDemandSet::Collect),
        );
}

fn collect_spatial_demand(
    view: Res<UsfViewFrame>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    sources: Query<(
        Entity,
        &GlobalTransform,
        &SpatialDemandSource,
        Option<&UsfScaleLayer>,
    )>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
) {
    snapshot.scopes.clear();
    let active_scale = active.scale();

    for (entity, transform, source, source_layer) in &sources {
        if !source.enabled() {
            continue;
        }
        let source_scale = source_layer.map_or(active_scale, |layer| layer.scale());
        let source_absolute = frames.absolute(source_scale, transform.translation());

        for demand in view.active_scale_demands().into_iter().flatten() {
            let target_scale = demand.scale();
            if target_scale < SpatialScale::ZERO || demand.contribution() <= 0.001 {
                continue;
            }

            let target_absolute =
                frames.convert_absolute(source_absolute, source_scale, target_scale);
            let local = Vec3::new(
                target_absolute.x as f32,
                target_absolute.y as f32,
                target_absolute.z as f32,
            );
            if !local.is_finite() {
                continue;
            }
            let Ok(center) = UsfPosition::zero(target_scale).translated_native(local) else {
                continue;
            };

            let half_extent = if target_scale == active_scale {
                source.half_extent_native()
            } else {
                source.half_extent_native() * 0.5
            };
            snapshot.scopes.push(SpatialDemandScope::at_scale(
                entity,
                target_scale,
                center,
                half_extent,
                source.priority(),
            ));
        }
    }
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
}
