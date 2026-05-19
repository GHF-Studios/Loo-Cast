use super::super::aliases::UsfOrNormal;
use super::super::scalar::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::normal::NormalVector;
use super::usf::UsfVector;
use base_mod_shared::utils::one_of::OneOf2;

pub type UsfOrNormalVector<const D: usize> = UsfOrNormal<UsfVector<D>, NormalVector<D>>;
pub type VectorProductOperand<const D: usize> = OneOf2<UsfOrNormalVector<D>, UsfOrNormalScalar>;
pub type VectorProductResult<const D: usize> = OneOf2<UsfOrNormalVector<D>, UsfOrNormalFractionalScalar>;
