//! Canonical position value, balanced digit arithmetic, and bounded projection.

use self::decimal::{format_decimal_terms, push_float_decimal_terms};
use crate::usf::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SpatialScale, USF_BALANCED_DIGIT_MAX_EXCLUSIVE,
    USF_BALANCED_DIGIT_MIN, USF_CHILD_CHUNKS_PER_AXIS, USF_CHUNK_NATIVE_SIZE,
    USF_LOCAL_MAX_EXCLUSIVE, USF_LOCAL_MIN,
};
use glam::{DVec3, IVec3, Vec3};
use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfPositionError {
    NonFiniteTranslation,
    TranslationTooLarge,
    IncompatibleLeafScale,
    RelativePositionOutsideBound,
}

/// Canonical decimal USF spatial position.
///
/// Each digit selects one of `10 x 10 x 10` child chunks using balanced digits
/// `[-5, 5)`. Every chunk spans `1000^3` units native to its own scale. The
/// final `offset` is measured in units native to `leaf_scale` and is normalized
/// into `[-500, 500)` on every axis.
///
/// `leaf_scale` is the finest currently resolved digit of this particular
/// canonical position. It may be anywhere in the full S-35..S+35 range; S0 has
/// no special positional-authority meaning.
#[derive(bevy::prelude::Component, Clone, Copy, PartialEq)]
pub struct UsfPosition {
    digits: [IVec3; SPATIAL_SCALE_COUNT],
    leaf_scale: SpatialScale,
    offset: Vec3,
}

/// One scalar axis of a canonical USF position.
///
/// Its ordinary decimal representation is expressed in S0 units (SI metres).
/// Scale Stack digits therefore become ordinary decimal digits before/after the
/// decimal point without collapsing the canonical value through a machine float.
#[derive(Clone, Copy, PartialEq)]
pub struct UsfCoordinate {
    digits: [i8; SPATIAL_SCALE_COUNT],
    leaf_scale: SpatialScale,
    offset: f32,
}

mod arithmetic;
mod construction;
mod decimal;
mod display;
mod relative;

// `UsfPosition` constructors and translation APIs reject non-finite offsets and
// normalize them into one canonical range, so semantic positions have a valid
// equivalence relation even though their bounded leaf offset is stored as f32.
impl Eq for UsfPosition {}

impl Hash for UsfPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for digit in &self.digits {
            digit.x.hash(state);
            digit.y.hash(state);
            digit.z.hash(state);
        }
        self.leaf_scale.exponent().hash(state);
        canonical_f32_bits(self.offset.x).hash(state);
        canonical_f32_bits(self.offset.y).hash(state);
        canonical_f32_bits(self.offset.z).hash(state);
    }
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

fn axis_f32(value: Vec3, axis: usize) -> f32 {
    match axis {
        0 => value.x,
        1 => value.y,
        2 => value.z,
        _ => unreachable!(),
    }
}

fn set_axis_f32(value: &mut Vec3, axis: usize, component: f32) {
    match axis {
        0 => value.x = component,
        1 => value.y = component,
        2 => value.z = component,
        _ => unreachable!(),
    }
}

fn axis_i32(value: IVec3, axis: usize) -> i32 {
    match axis {
        0 => value.x,
        1 => value.y,
        2 => value.z,
        _ => unreachable!(),
    }
}

fn set_axis_i32(value: &mut IVec3, axis: usize, component: i32) {
    match axis {
        0 => value.x = component,
        1 => value.y = component,
        2 => value.z = component,
        _ => unreachable!(),
    }
}
