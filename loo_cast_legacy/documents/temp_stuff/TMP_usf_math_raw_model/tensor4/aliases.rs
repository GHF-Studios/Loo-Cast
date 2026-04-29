#![allow(dead_code)]

use super::super::aliases::UsfOrNormal;
use super::normal::NormalTensor4;
use super::usf::UsfTensor4;
use crate::utils::one_of::OneOf2;

use super::super::scalar::aliases::UsfOrNormalScalar;

pub type UsfOrNormalTensor4<const A: usize, const B: usize, const C: usize, const D: usize> = UsfOrNormal<UsfTensor4<A, B, C, D>, NormalTensor4<A, B, C, D>>;
pub type Tensor4OrScalar<const A: usize, const B: usize, const C: usize, const D: usize> = OneOf2<UsfOrNormalTensor4<A, B, C, D>, UsfOrNormalScalar>;
