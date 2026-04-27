#![allow(dead_code)]

use super::super::aliases::UsfOrNormal;
use super::normal::{NormalFractionalScalar, NormalScalar};
use super::usf::UsfScalar;

/// Mixed-repr scalar alias for general scalar operands.
pub type UsfOrNormalScalar = UsfOrNormal<UsfScalar, NormalScalar>;

/// Mixed-repr scalar alias for fractional-capable scalar operands/results.
/// This alias is capability-oriented:
/// - `UsfScalar` remains valid even when the runtime value is integer-like.
/// - `NormalFractionalScalar` captures fractional-capable normal-repr outputs.
pub type UsfOrNormalFractionalScalar = UsfOrNormal<UsfScalar, NormalFractionalScalar>;
