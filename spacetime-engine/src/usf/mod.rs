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
    pub fn metres_per_native(self) -> f64 {
        10.0_f64.powi(self.exponent() as i32)
    }

    /// Projects an SI-metre distance/speed/acceleration into this slice's native units.
    pub fn metres_to_native_f64(self, value: f64) -> f64 {
        value / self.metres_per_native()
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
