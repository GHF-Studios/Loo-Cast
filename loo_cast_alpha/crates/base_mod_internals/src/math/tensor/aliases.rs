use super::super::aliases::UsfOrNormal;
use super::normal::NormalTensor;
use super::usf::UsfTensor;
use base_mod_shared::utils::one_of::OneOf2;

use super::super::scalar::aliases::UsfOrNormalScalar;

pub type UsfOrNormalTensor<const A: usize, const B: usize, const C: usize> = UsfOrNormal<UsfTensor<A, B, C>, NormalTensor<A, B, C>>;
pub type TensorOrScalar<const A: usize, const B: usize, const C: usize> = OneOf2<UsfOrNormalTensor<A, B, C>, UsfOrNormalScalar>;
