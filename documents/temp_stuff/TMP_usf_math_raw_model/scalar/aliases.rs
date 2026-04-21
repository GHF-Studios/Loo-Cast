#![allow(dead_code)]

use super::super::aliases::UsfOrNormal;
use super::normal::{NormalFractionalScalar, NormalScalar};
use super::usf::UsfScalar;

/// Mixed-domain scalar alias for general scalar operands.
pub type UsfOrNormalScalar = UsfOrNormal<UsfScalar, NormalScalar>;

/// Mixed-domain scalar alias for fractional-capable scalar operands/results.
/// This alias is capability-oriented:
/// - `UsfScalar` remains valid even when the runtime value is integer-like.
/// - `NormalFractionalScalar` captures fractional-capable normal-domain outputs.
pub type UsfOrNormalFractionalScalar = UsfOrNormal<UsfScalar, NormalFractionalScalar>;
