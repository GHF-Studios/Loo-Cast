//! Quantized predictive motion as work priority, not authority.

use super::*;

const MIN_PREDICTIVE_VALIDITY_SECONDS: f64 = 1.0;
const MAX_PREDICTIVE_VALIDITY_SECONDS: f64 = 4.0;
const PREDICTIVE_LATENCY_MULTIPLIER: f64 = 4.0;
const MOTION_DIRECTION_QUANTIZATION: f64 = 8.0;
// Predictive depth may extend far forward, but the immediate local/contact
// footprint must re-anchor on every materialization-boundary crossing.
pub(super) const MOVING_PLAN_CENTER_HOLD_CHUNKS: u64 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelMotionPriorityKey {
    pub(super) direction: [i8; 3],
    pub(super) speed_bucket: u8,
    pub(super) horizon_bucket: u8,
}

impl VoxelMotionPriorityKey {
    pub(super) const STATIONARY: Self = Self {
        direction: [0, 0, 0],
        speed_bucket: 0,
        horizon_bucket: 0,
    };
}

#[derive(Debug, Clone, Copy)]
pub(super) struct VoxelDemandMotion {
    pub(super) predicted_offset_native: Vec3,
    pub(super) lookahead_seconds: f32,
    pub(super) bias: f32,
    pub(super) direction_native: Vec3,
}

impl VoxelDemandMotion {
    pub(super) fn with_expected_latency(
        demand: SpatialDemandScope,
        velocity_metres_per_second: DVec3,
        expected_build_seconds: f64,
    ) -> Self {
        if !velocity_metres_per_second.is_finite() {
            return Self::stationary();
        }
        let factor = demand.scale().scale0_to_native_f64(1.0);
        let native = velocity_metres_per_second * factor;
        let velocity_native = Vec3::new(
            saturating_motion_f32(native.x),
            saturating_motion_f32(native.y),
            saturating_motion_f32(native.z),
        );
        let speed_native = velocity_native.length();
        if !speed_native.is_finite() || speed_native <= f32::EPSILON {
            return Self::stationary();
        }

        let speed_chunks = speed_native / MATERIALIZATION_CHUNK_SIZE as f32;
        let bias = (speed_chunks / (speed_chunks + 1.0)).clamp(0.0, 1.0);
        let chunk_extent_metres =
            f64::from(MATERIALIZATION_CHUNK_SIZE) * demand.scale().metres_per_native();
        let granularity = SpatialRealizationGranularityRequest::new(
            chunk_extent_metres,
            chunk_extent_metres,
            chunk_extent_metres,
            1,
            1,
            velocity_metres_per_second.length(),
            expected_build_seconds,
            MIN_PREDICTIVE_VALIDITY_SECONDS,
            MAX_PREDICTIVE_VALIDITY_SECONDS,
            PREDICTIVE_LATENCY_MULTIPLIER,
        )
        .solve();
        let lookahead_seconds = granularity.validity_seconds() as f32;
        let predicted_offset_native = velocity_native * lookahead_seconds;

        Self {
            predicted_offset_native,
            lookahead_seconds,
            bias,
            direction_native: velocity_native.normalize_or_zero(),
        }
    }

    const fn stationary() -> Self {
        Self {
            predicted_offset_native: Vec3::ZERO,
            lookahead_seconds: 0.0,
            bias: 0.0,
            direction_native: Vec3::ZERO,
        }
    }

    pub(super) fn trajectory_distance_squared(self, relative: Vec3) -> f32 {
        let current = relative.length_squared();
        if self.bias <= f32::EPSILON {
            return current;
        }
        let segment = self.predicted_offset_native;
        let segment_length_squared = segment.length_squared();
        if segment_length_squared <= f32::EPSILON {
            return current;
        }
        let t = (relative.dot(segment) / segment_length_squared).clamp(0.0, 1.0);
        let nearest = segment * t;
        let lateral_squared = (relative - nearest).length_squared();
        let corridor_score = lateral_squared * 8.0 + current * 0.05;
        (current + (corridor_score - current) * self.bias).max(0.0)
    }

    pub(super) fn demand_offsets(self, half_extent: Vec3) -> (Vec3, Vec3) {
        let minimum = -half_extent;
        let maximum = half_extent;
        if self.bias <= f32::EPSILON || self.direction_native == Vec3::ZERO {
            return (minimum, maximum);
        }
        (
            minimum.min(self.predicted_offset_native - half_extent),
            maximum.max(self.predicted_offset_native + half_extent),
        )
    }
}

pub(super) fn quantized_motion_key(
    velocity_metres_per_second: DVec3,
    lookahead_seconds: f32,
) -> VoxelMotionPriorityKey {
    let speed = velocity_metres_per_second.length();
    if !speed.is_finite() || speed < 0.5 {
        return VoxelMotionPriorityKey::STATIONARY;
    }
    let direction = velocity_metres_per_second / speed;
    let quantize = |value: f64| {
        (value * MOTION_DIRECTION_QUANTIZATION).round().clamp(
            -MOTION_DIRECTION_QUANTIZATION,
            MOTION_DIRECTION_QUANTIZATION,
        ) as i8
    };
    let speed_bucket = (speed.log2().floor() + 16.0).clamp(1.0, 63.0) as u8;
    let horizon_bucket = if lookahead_seconds > 0.0 {
        (f64::from(lookahead_seconds).log2().floor() + 16.0).clamp(1.0, 63.0) as u8
    } else {
        0
    };
    VoxelMotionPriorityKey {
        direction: [
            quantize(direction.x),
            quantize(direction.y),
            quantize(direction.z),
        ],
        speed_bucket,
        horizon_bucket,
    }
}

fn saturating_motion_f32(value: f64) -> f32 {
    if value.is_nan() {
        0.0
    } else {
        value.clamp(-f64::from(f32::MAX), f64::from(f32::MAX)) as f32
    }
}
