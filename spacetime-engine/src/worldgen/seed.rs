//! Deterministic world-generation hashing/noise helpers.

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfChunkAddress};

use super::model::PhenomenonEvaluationContext;

pub(super) fn scope_seed(universe_seed: u64, scope: UsfChunkAddress) -> u64 {
    let mut state = mix64(universe_seed ^ (scope.scale().exponent() as i64 as u64));
    for raw_scale in (scope.scale().exponent()..=SPATIAL_SCALE_MAX).rev() {
        let scale = SpatialScale::new(raw_scale).expect("validated spatial scale range");
        let digit = scope
            .digit(scale)
            .expect("address stores every digit at and above its scale");
        state = mix64(state ^ (digit.x as i64 as u64).wrapping_mul(0x9E37_79B9));
        state = mix64(state ^ (digit.y as i64 as u64).wrapping_mul(0x85EB_CA6B));
        state = mix64(state ^ (digit.z as i64 as u64).wrapping_mul(0xC2B2_AE35));
    }
    state
}

fn unit_noise(context: &PhenomenonEvaluationContext, salt: u64) -> f32 {
    let value = mix64(context.seed() ^ salt);
    let unit = (value >> 11) as f64 / ((1_u64 << 53) - 1) as f64;
    unit as f32
}

pub(super) fn signed_noise(context: &PhenomenonEvaluationContext, salt: u64) -> f32 {
    unit_noise(context, salt) * 2.0 - 1.0
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;
    value
}
