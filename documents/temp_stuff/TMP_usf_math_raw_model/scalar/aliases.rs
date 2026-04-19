#![allow(dead_code)]

use super::super::aliases::UsfOrNormal;
use super::normal::{NormalDecimalScalar, NormalScalar};
use super::usf::UsfScalar;

pub type UsfOrNormalScalar = UsfOrNormal<UsfScalar, NormalScalar>;
pub type UsfOrNormalDecimalScalar = UsfOrNormal<UsfScalar, NormalDecimalScalar>;
