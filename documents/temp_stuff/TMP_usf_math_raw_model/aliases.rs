#![allow(dead_code)]

use super::matrix::{normal::NormalMatrix, usf::UsfMatrix};
use super::quaternion::{normal::NormalQuaternion, usf::UsfQuaternion};
use super::scalar::{
    normal::{NormalDecimalScalar, NormalScalar},
    usf::UsfScalar,
};
use super::vector::{normal::NormalVector, usf::UsfVector};
use crate::utils::one_of::OneOf2;

pub type UsfOrNormal<UsfT, NormalT> = OneOf2<UsfT, NormalT>;

pub type UsfOrNormalScalar = UsfOrNormal<UsfScalar, NormalScalar>;
pub type UsfOrNormalDecimalScalar = UsfOrNormal<UsfScalar, NormalDecimalScalar>;

pub type UsfOrNormalVector<const D: usize> = UsfOrNormal<UsfVector<D>, NormalVector<D>>;
pub type UsfOrNormalMatrix<const R: usize, const C: usize> = UsfOrNormal<UsfMatrix<R, C>, NormalMatrix<R, C>>;
pub type UsfOrNormalMat3 = UsfOrNormal<UsfMatrix<3, 3>, NormalMatrix<3, 3>>;
pub type UsfOrNormalQuaternion = UsfOrNormal<UsfQuaternion, NormalQuaternion>;
