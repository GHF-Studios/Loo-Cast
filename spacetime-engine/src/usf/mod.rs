//! Canonical Universal Simulation Framework spatial-number algebra.
//!
//! This module owns the hierarchical Scale Stack number itself: spatial scales,
//! balanced decimal digits, normalization/carry, canonical displacement,
//! re-expression, canonical topology, and bounded chart mathematics.
//!
//! USF is an engine-core module, not a separately distributed Cargo package.
//! Runtime Scale Slices, residency/refinement, semantic realization, physics,
//! rendering and world policy remain downstream responsibilities. Bevy derives
//! on canonical value types are host metadata, not ownership of those policies.
//!
//! ## Module map
//!
//! - `chunk_address`: Canonical identity and traversal of one USF chunk at one spatial scale.
//! - `chart`: Pure bounded chart algebra over canonical USF space.
//! - `position`: Canonical position value, balanced digit arithmetic, and bounded projection.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::fmt::{Display, Formatter};

mod chunk_address;
pub use chunk_address::UsfChunkAddress;
mod chart;
pub use chart::{UsfChart, UsfChartDelta};

pub const SPATIAL_SCALE_MAX: i8 = 35;
pub const SPATIAL_SCALE_MIN: i8 = -35;
pub const SPATIAL_SCALE_COUNT: usize =
    (SPATIAL_SCALE_MAX as i16 - SPATIAL_SCALE_MIN as i16 + 1) as usize;

// There are only 71 legal decimal Scales. Hot code should never recompute
// these powers or divide by them.
const METRES_PER_NATIVE: [f64; SPATIAL_SCALE_COUNT] = [1.0e35, 1.0e34, 1.0e33, 1.0e32, 1.0e31, 1.0e30, 1.0e29, 1.0e28, 1.0e27, 1.0e26, 1.0e25, 1.0e24, 1.0e23, 1.0e22, 1.0e21, 1.0e20, 1.0e19, 1.0e18, 1.0e17, 1.0e16, 1.0e15, 1.0e14, 1.0e13, 1.0e12, 1.0e11, 1.0e10, 1.0e9, 1.0e8, 1.0e7, 1.0e6, 1.0e5, 1.0e4, 1.0e3, 1.0e2, 1.0e1, 1.0e0, 1.0e-1, 1.0e-2, 1.0e-3, 1.0e-4, 1.0e-5, 1.0e-6, 1.0e-7, 1.0e-8, 1.0e-9, 1.0e-10, 1.0e-11, 1.0e-12, 1.0e-13, 1.0e-14, 1.0e-15, 1.0e-16, 1.0e-17, 1.0e-18, 1.0e-19, 1.0e-20, 1.0e-21, 1.0e-22, 1.0e-23, 1.0e-24, 1.0e-25, 1.0e-26, 1.0e-27, 1.0e-28, 1.0e-29, 1.0e-30, 1.0e-31, 1.0e-32, 1.0e-33, 1.0e-34, 1.0e-35];
const NATIVE_PER_METRE: [f64; SPATIAL_SCALE_COUNT] = [1.0e-35, 1.0e-34, 1.0e-33, 1.0e-32, 1.0e-31, 1.0e-30, 1.0e-29, 1.0e-28, 1.0e-27, 1.0e-26, 1.0e-25, 1.0e-24, 1.0e-23, 1.0e-22, 1.0e-21, 1.0e-20, 1.0e-19, 1.0e-18, 1.0e-17, 1.0e-16, 1.0e-15, 1.0e-14, 1.0e-13, 1.0e-12, 1.0e-11, 1.0e-10, 1.0e-9, 1.0e-8, 1.0e-7, 1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3, 1.0e-2, 1.0e-1, 1.0e0, 1.0e1, 1.0e2, 1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7, 1.0e8, 1.0e9, 1.0e10, 1.0e11, 1.0e12, 1.0e13, 1.0e14, 1.0e15, 1.0e16, 1.0e17, 1.0e18, 1.0e19, 1.0e20, 1.0e21, 1.0e22, 1.0e23, 1.0e24, 1.0e25, 1.0e26, 1.0e27, 1.0e28, 1.0e29, 1.0e30, 1.0e31, 1.0e32, 1.0e33, 1.0e34, 1.0e35];
const POW10_SCALE_DELTA: [f64; SPATIAL_SCALE_COUNT] = [1.0e0, 1.0e1, 1.0e2, 1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7, 1.0e8, 1.0e9, 1.0e10, 1.0e11, 1.0e12, 1.0e13, 1.0e14, 1.0e15, 1.0e16, 1.0e17, 1.0e18, 1.0e19, 1.0e20, 1.0e21, 1.0e22, 1.0e23, 1.0e24, 1.0e25, 1.0e26, 1.0e27, 1.0e28, 1.0e29, 1.0e30, 1.0e31, 1.0e32, 1.0e33, 1.0e34, 1.0e35, 1.0e36, 1.0e37, 1.0e38, 1.0e39, 1.0e40, 1.0e41, 1.0e42, 1.0e43, 1.0e44, 1.0e45, 1.0e46, 1.0e47, 1.0e48, 1.0e49, 1.0e50, 1.0e51, 1.0e52, 1.0e53, 1.0e54, 1.0e55, 1.0e56, 1.0e57, 1.0e58, 1.0e59, 1.0e60, 1.0e61, 1.0e62, 1.0e63, 1.0e64, 1.0e65, 1.0e66, 1.0e67, 1.0e68, 1.0e69, 1.0e70];
const INV_POW10_SCALE_DELTA: [f64; SPATIAL_SCALE_COUNT] = [1.0e0, 1.0e-1, 1.0e-2, 1.0e-3, 1.0e-4, 1.0e-5, 1.0e-6, 1.0e-7, 1.0e-8, 1.0e-9, 1.0e-10, 1.0e-11, 1.0e-12, 1.0e-13, 1.0e-14, 1.0e-15, 1.0e-16, 1.0e-17, 1.0e-18, 1.0e-19, 1.0e-20, 1.0e-21, 1.0e-22, 1.0e-23, 1.0e-24, 1.0e-25, 1.0e-26, 1.0e-27, 1.0e-28, 1.0e-29, 1.0e-30, 1.0e-31, 1.0e-32, 1.0e-33, 1.0e-34, 1.0e-35, 1.0e-36, 1.0e-37, 1.0e-38, 1.0e-39, 1.0e-40, 1.0e-41, 1.0e-42, 1.0e-43, 1.0e-44, 1.0e-45, 1.0e-46, 1.0e-47, 1.0e-48, 1.0e-49, 1.0e-50, 1.0e-51, 1.0e-52, 1.0e-53, 1.0e-54, 1.0e-55, 1.0e-56, 1.0e-57, 1.0e-58, 1.0e-59, 1.0e-60, 1.0e-61, 1.0e-62, 1.0e-63, 1.0e-64, 1.0e-65, 1.0e-66, 1.0e-67, 1.0e-68, 1.0e-69, 1.0e-70];

pub const USF_CHUNK_NATIVE_SIZE: f32 = 1000.0;
pub const USF_CHILD_CHUNKS_PER_AXIS: i32 = 10;
pub const USF_BALANCED_DIGIT_MIN: i32 = -5;
pub const USF_BALANCED_DIGIT_MAX_EXCLUSIVE: i32 = 5;
pub const USF_LOCAL_MIN: f32 = -USF_CHUNK_NATIVE_SIZE * 0.5;
pub const USF_LOCAL_MAX_EXCLUSIVE: f32 = USF_CHUNK_NATIVE_SIZE * 0.5;

#[derive(bevy::reflect::Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpatialScale(i8);

impl SpatialScale {
    pub const MIN: Self = Self(SPATIAL_SCALE_MIN);
    pub const MAX: Self = Self(SPATIAL_SCALE_MAX);
    pub const ZERO: Self = Self(0);

    pub const fn new(value: i8) -> Option<Self> {
        if value >= SPATIAL_SCALE_MIN && value <= SPATIAL_SCALE_MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    pub const fn exponent(self) -> i8 {
        self.0
    }

    pub const fn index_from_top(self) -> usize {
        (SPATIAL_SCALE_MAX - self.0) as usize
    }

    /// SI metres represented by one native unit in this Scale Slice.
    ///
    /// The fact that S0's native spatial unit equals one metre is a unit-system
    /// convention only. S0 is not an architectural origin, center, minimum, or
    /// otherwise privileged Scale Slice.
    pub const fn metres_per_native(self) -> f64 {
        METRES_PER_NATIVE[self.index_from_top()]
    }

    /// Native units represented by one SI metre in this Scale Slice.
    pub const fn native_per_metre(self) -> f64 {
        NATIVE_PER_METRE[self.index_from_top()]
    }

    /// Multiplicative conversion from this Scale's native units to `target`.
    ///
    /// Decimal Scale relations are fixed structure, so hot callers can multiply
    /// once instead of evaluating powi/division.
    pub const fn native_to_native_factor(self, target: Self) -> f64 {
        let delta = self.0 as i16 - target.0 as i16;
        if delta >= 0 {
            POW10_SCALE_DELTA[delta as usize]
        } else {
            INV_POW10_SCALE_DELTA[(-delta) as usize]
        }
    }

    /// Projects an SI-metre distance/speed/acceleration into this slice's native units.
    pub fn metres_to_native_f64(self, value: f64) -> f64 {
        value * self.native_per_metre()
    }

    /// f32 adapter for bounded chart-local runtime APIs.
    pub fn metres_to_native_f32(self, value: f32) -> f32 {
        self.metres_to_native_f64(f64::from(value))
            .clamp(-(f32::MAX as f64), f32::MAX as f64) as f32
    }

    /// Converts one chart-native runtime value back to SI metres.
    pub fn native_to_metres_f32(self, value: f32) -> f32 {
        (f64::from(value) * self.metres_per_native()).clamp(-(f32::MAX as f64), f32::MAX as f64)
            as f32
    }
}

impl Display for SpatialScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self.0)
    }
}

mod position;
pub use position::{UsfCoordinate, UsfPosition, UsfPositionError};
