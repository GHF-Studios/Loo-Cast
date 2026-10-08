//! Canonical volumetric celestial field and bounded presentation sampling.
//!
//! The canonical Scale Stack carries the enormous position/range. Dense voxel
//! sampling sees a bounded local chart around one canonical surface anchor.
//!
//! ## Module map
//!
//! - `field`: Canonical celestial field and its bounded runtime sampling adapter.
//! - `sampling`: Prepared body-local field sampling.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::{
    math::{DQuat, DVec3},
    prelude::Vec3,
};

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError, UsfSemanticFrame};

use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};
use super::EMPTY_DISTANCE;

mod field;
mod sampling;

pub(crate) use sampling::PreparedCelestialBodySampler;

/// One canonical body-local volumetric field sample.
///
/// This is semantic terrain truth. Realization Scale, render LOD, dense cache
/// address and collision backend are all downstream adapters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CelestialFieldSample {
    signed_distance_metres: f64,
    material: VoxelMaterialId,
}

impl CelestialFieldSample {
    pub(crate) const fn new(signed_distance_metres: f64, material: VoxelMaterialId) -> Self {
        Self {
            signed_distance_metres,
            material,
        }
    }

    pub(crate) const fn signed_distance_metres(self) -> f64 {
        self.signed_distance_metres
    }
}

/// Reconstructible celestial field.
///
/// Semantic radius stays in SI `f64`; bounded charts resolve a surface anchor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CelestialFieldRealization {
    origin_snapshot: UsfPosition,
    frame_snapshot: UsfSemanticFrame,
    radius_metres: f64,
    /// Numerical/cache chart only.
    realization_scale: SpatialScale,
}

/// Prepared celestial sampling stores one exact body-local SI chunk origin.
///
/// Scale chooses only how large each `chunk_local` step is in metres. It never
/// selects a different terrain algorithm.
#[derive(Debug)]
pub(crate) struct PreparedCelestialVoxelSampler {
    body: PreparedCelestialBodySampler,
    chunk_origin_local_metres: DVec3,
    world_to_local: DQuat,
    metres_per_native: f64,
    native_per_metre: f64,
}

fn normalized_direction(direction: Vec3) -> Vec3 {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO {
        Vec3::Y
    } else {
        direction
    }
}

fn dvec(value: Vec3) -> DVec3 {
    DVec3::new(f64::from(value.x), f64::from(value.y), f64::from(value.z))
}
