#![allow(dead_code)]

use super::super::aliases::UsfOrNormal;
use super::normal::NormalVector;
use super::usf::UsfVector;
use crate::utils::one_of::OneOf2;

use super::super::scalar::usf::UsfScalar;

pub type UsfOrNormalVector<const D: usize> = UsfOrNormal<UsfVector<D>, NormalVector<D>>;
pub type VectorOrScalar<const D: usize> = OneOf2<UsfVector<D>, UsfScalar>;
