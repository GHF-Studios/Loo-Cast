#![allow(dead_code)]

use super::super::aliases::UsfOrNormal;
use super::normal::NormalMatrix;
use super::usf::UsfMatrix;
use crate::utils::one_of::OneOf2;

use super::super::scalar::usf::UsfScalar;
use super::super::vector::usf::UsfVector;

pub type UsfOrNormalMatrix<const R: usize, const C: usize> = UsfOrNormal<UsfMatrix<R, C>, NormalMatrix<R, C>>;
pub type MatrixOrScalar<const R: usize, const C: usize> = OneOf2<UsfMatrix<R, C>, UsfScalar>;
pub type MatrixOrVector<const R: usize, const C: usize> = OneOf2<UsfMatrix<R, C>, UsfVector<C>>;
