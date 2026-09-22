//! The 71 explicit USF Scale Slices and scale-local runtime membership.
//!
//! Each layer lets ordinary engine coordinates mean "native units at this USF
//! scale". Physics/render/audio/etc. adapters can share this identity without
//! teaching every low-level engine about astronomical unit magnitudes.

use bevy::{math::DVec3, prelude::*};

use super::{SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MIN, SpatialScale};


/// One of the 71 fundamental spatial Scale Slices.
///
/// A slice is not an LOD level and not a separate universe. It is one
/// scale-local runtime partition of the same canonical game world.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfScaleSlice {
    scale: SpatialScale,
}
impl UsfScaleSlice {
    pub const fn new(scale: SpatialScale) -> Self { Self { scale } }
    pub const fn scale(self) -> SpatialScale { self.scale }
}

/// Runtime registry of the 71 Scale Slice roots.
#[derive(Resource, Debug)]
pub struct UsfScaleSlices {
    roots: [Option<Entity>; SPATIAL_SCALE_COUNT],
}
impl Default for UsfScaleSlices {
    fn default() -> Self { Self { roots: [None; SPATIAL_SCALE_COUNT] } }
}
impl UsfScaleSlices {
    pub fn root(&self, scale: SpatialScale) -> Option<Entity> {
        self.roots[scale.index_from_top()]
    }
    pub fn iter(&self) -> impl Iterator<Item=(SpatialScale, Entity)> + '_ {
        (SPATIAL_SCALE_MIN..=super::SPATIAL_SCALE_MAX).filter_map(|raw| {
            let scale=SpatialScale::new(raw)?;
            self.root(scale).map(|entity|(scale,entity))
        })
    }
}
pub(in crate::spatial) fn spawn_scale_slices(
    mut commands: Commands,
    mut slices: ResMut<UsfScaleSlices>,
) {
    for raw in SPATIAL_SCALE_MIN..=super::SPATIAL_SCALE_MAX {
        let scale=SpatialScale::new(raw).expect("validated USF scale");
        let index=scale.index_from_top();
        if slices.roots[index].is_some() { continue; }
        let entity=commands.spawn((
            Name::new(format!("USF Scale Slice S{scale}")),
            UsfScaleSlice::new(scale),
        )).id();
        slices.roots[index]=Some(entity);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfScaleLayer {
    scale: SpatialScale,
}

impl UsfScaleLayer {
    pub const fn new(scale: SpatialScale) -> Self {
        Self { scale }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn chart_mask(self) -> UsfChartMask {
        UsfChartMask::from_scale(self.scale)
    }

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

/// A set of USF simulation charts.
///
/// There are 71 spatial scales, so one `u128` contains the entire chart set
/// without borrowing Avian's finite collision-category layer mask.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfChartMask(u128);

impl UsfChartMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self((1_u128 << SPATIAL_SCALE_COUNT) - 1);

    pub const fn from_scale(scale: SpatialScale) -> Self {
        let bit = (scale.exponent() as i16 - SPATIAL_SCALE_MIN as i16) as u32;
        Self(1_u128 << bit)
    }

    pub const fn bits(self) -> u128 {
        self.0
    }

    pub const fn contains(self, scale: SpatialScale) -> bool {
        (self.0 & Self::from_scale(scale).0) != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

/// Transitional compatibility marker for manifestations whose primary
/// interaction slice may be explicitly rechosen.
///
/// IMPORTANT: this does not mean only one Scale Slice exists.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfFollowsActiveScale;

/// Compatibility resource naming the primary controlled interaction slice.
///
/// It is not "the active world". All 71 slices exist simultaneously.
#[derive(Resource, Debug, Clone, Copy)]
pub struct UsfActiveScaleLayer {
    scale: SpatialScale,
}

impl Default for UsfActiveScaleLayer {
    fn default() -> Self {
        Self {
            scale: SpatialScale::MAX,
        }
    }
}

impl UsfActiveScaleLayer {
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

/// Runtime origin stack for every scale-local world.
///
/// Each scale is an ordinary bounded floating-point chart with its own origin.
/// Physics, rendering and other local engine subsystems may interpret the same
/// numeric coordinates in that scale's native units without ever constructing
/// universe-wide floats.
///
/// These origins are projection context, never semantic authority. Rebasing one
/// scale therefore never shifts another scale's local world.
#[derive(Resource, Debug, Clone)]
pub struct UsfScaleLayerFrames {
    origins: [DVec3; SPATIAL_SCALE_COUNT],
}

impl Default for UsfScaleLayerFrames {
    fn default() -> Self {
        Self {
            origins: [DVec3::ZERO; SPATIAL_SCALE_COUNT],
        }
    }
}

impl UsfScaleLayerFrames {
    pub fn origin(&self, scale: SpatialScale) -> DVec3 {
        self.origins[scale.index_from_top()]
    }

    pub fn absolute(&self, scale: SpatialScale, runtime: Vec3) -> DVec3 {
        self.origin(scale) + to_dvec3(runtime)
    }

    pub fn convert_absolute(&self, absolute: DVec3, from: SpatialScale, to: SpatialScale) -> DVec3 {
        let factor = 10.0_f64.powi(from.exponent() as i32 - to.exponent() as i32);
        absolute * factor
    }

    pub fn runtime_from_absolute(&self, scale: SpatialScale, absolute: DVec3) -> Vec3 {
        to_vec3(absolute - self.origin(scale))
    }

    pub(crate) fn set_origin(&mut self, scale: SpatialScale, origin: DVec3) {
        self.origins[scale.index_from_top()] = origin;
    }

    pub fn reinterpret_runtime(&self, runtime: Vec3, from: SpatialScale, to: SpatialScale) -> Vec3 {
        let absolute = self.absolute(from, runtime);
        let converted = self.convert_absolute(absolute, from, to);
        self.runtime_from_absolute(to, converted)
    }

    pub(crate) fn apply_rebase(&mut self, scale: SpatialScale, shift: Vec3) {
        self.origins[scale.index_from_top()] += to_dvec3(shift);
    }
}

fn to_dvec3(value: Vec3) -> DVec3 {
    DVec3::new(value.x as f64, value.y as f64, value.z as f64)
}

fn to_vec3(value: DVec3) -> Vec3 {
    Vec3::new(value.x as f32, value.y as f32, value.z as f32)
}


#[cfg(test)]
mod chart_mask_tests {
    use super::*;

    #[test]
    fn chart_mask_covers_every_spatial_scale_once() {
        let mut accumulated = UsfChartMask::NONE;
        for raw in super::super::SPATIAL_SCALE_MIN..=super::super::SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw).unwrap();
            let mask = UsfChartMask::from_scale(scale);
            assert_ne!(mask.bits(), 0);
            assert!(!accumulated.intersects(mask));
            accumulated = accumulated.union(mask);
        }
        assert_eq!(accumulated, UsfChartMask::ALL);
    }
}
