//! Procedural voxel baselines for generic celestial bodies.
//!
//! The canonical Scale Stack carries the enormous position/range. Dense voxel
//! sampling only sees a bounded local chart around one canonical surface anchor.
//!
//! ## Module map
//!
//! - `bands`: Semantic-scale descriptors for planetary terrain morphology.
//! - `caves`: Deterministic volumetric void morphology for rocky celestial bodies.
//! - `field`: Canonical celestial field and its bounded runtime sampling adapter.
//! - `presentation`: Prepared, cache-backed celestial presentation sampling.
//! - `profiles`: Lunar and stellar macro relief profiles.
//! - `residual`: Shared residual noise law for exact and prepared celestial sampling.
//! - `rocky`: Hierarchical rocky-planet morphology.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::{math::DVec3, prelude::Vec3};

use crate::spatial::{
    SPATIAL_SCALE_COUNT, SpatialScale, UsfPosition, UsfPositionError, UsfSemanticFrame,
};

use super::super::{VoxelMaterialId, VoxelQueryPosition, VoxelSample};
use super::{
    EMPTY_DISTANCE,
    noise::{
        PreparedSemanticNoisePoint, SemanticNoiseCornerCache,
        record_fine_residual_fast_path_completion, record_fine_residual_generic_fallback,
        scale_layer_seed, semantic_value_noise_3d, semantic_value_noise_3d_cached,
        semantic_value_noise_3d_cached_prepared, value_noise_3d,
    },
};

mod bands;
mod caves;
mod field;
mod presentation;
mod profiles;
mod residual;
mod rocky;

pub(crate) use caves::{CAVE_MAX_DEPTH_METRES, CAVE_START_DEPTH_METRES};
use caves::{
    rocky_cave_void_may_intersect_aabb, rocky_cave_void_signed_distance_metres,
    rocky_cave_void_signed_distance_metres_with_radial,
};
pub(crate) use presentation::PreparedCelestialPresentationBody;
use profiles::{lunar_macro_relative_relief, stellar_macro_relative_relief};
use residual::{blend_residual_noise, coarse_residual_sample};
use rocky::{rocky_macro_displacement_metres, rocky_macro_relief_bounds_metres};

const LOCAL_SAMPLE_MARGIN_NATIVE: f32 = 8_192.0;
const LOCAL_SAMPLE_RELIEF_MARGIN_FRACTION: f64 = 0.05;
const CANONICAL_DETAIL_CELL_NATIVE: i64 = 20;
const CANONICAL_DETAIL_FINE_CELL_NATIVE: i64 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelestialBodyProfile {
    Lunar,
    Rocky,
    Stellar,
}

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

    pub(crate) const fn material(self) -> VoxelMaterialId {
        self.material
    }
}

/// Reconstructible celestial field.
///
/// Semantic radius stays in SI `f64`; it is never converted into one fine-slice
/// `f32` radius. S0 and finer instead resolve a canonical surface anchor first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CelestialFieldRealization {
    origin_snapshot: UsfPosition,
    frame_snapshot: UsfSemanticFrame,
    radius_metres: f64,
    /// Numerical/cache chart only.
    realization_scale: SpatialScale,
    coarsest_detail_scale: SpatialScale,
    /// Field-owned semantic surface bandwidth.
    surface_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

/// Prepared celestial sampling stores one exact body-local SI chunk origin.
///
/// Scale chooses only how large each `chunk_local` step is in metres. It never
/// selects a different terrain algorithm.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedCelestialVoxelSampler {
    body: CelestialFieldRealization,
    chunk_origin_local_metres: DVec3,
}

//
// Presentation samples the same body's full residual stack 729 times per
// central block. Resolve all Scale-dependent invariants once per sampler:
// SpatialScale construction, metres/native, growth.powi(), angular frequency
// and keyed seed leave the inner SDF loop.
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
