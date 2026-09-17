//! Scale-local simulation layers.
//!
//! Each layer lets ordinary engine coordinates mean "native units at this USF
//! scale". Physics/render/audio/etc. adapters can share this identity without
//! teaching every low-level engine about astronomical unit magnitudes.

use bevy::{math::DVec3, prelude::*};

use super::{SPATIAL_SCALE_COUNT, SpatialScale};

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

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

/// Marks a runtime manifestation that should move from one simulation layer to
/// another when the observer changes the dominant interactive scale.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfFollowsActiveScale;

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

/// Runtime chart origin for every possible scale-local world.
///
/// These are simulation/presentation chart coordinates, not semantic authority.
/// Keeping one origin per scale means rebasing S+35 never shifts S+34 geometry.
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
