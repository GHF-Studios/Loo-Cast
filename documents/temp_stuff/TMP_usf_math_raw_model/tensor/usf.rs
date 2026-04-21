#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::field::Field;
use super::super::matrix::aliases::UsfOrNormalMatrix;
use super::super::matrix::usf::UsfMatrix;
use super::super::scalar::aliases::UsfOrNormalScalar;
use super::super::scalar::usf::UsfScalar;
use super::super::vector::aliases::UsfOrNormalVector;
use super::super::vector::usf::UsfVector;
pub use super::aliases::{TensorOrScalar, UsfOrNormalTensor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTensor<const A: usize, const B: usize, const C: usize> {
    // CONTRACT: A,B,C >= 2. Any axis == 1 is reducible and forbidden.
    pub(super) slices: Field<[UsfMatrix<B, C>; A]>,
}

pub type Tensor2x2x2 = UsfTensor<2, 2, 2>;
pub type Tensor2x2x3 = UsfTensor<2, 2, 3>;
pub type Tensor2x3x3 = UsfTensor<2, 3, 3>;
pub type Tensor2x3x4 = UsfTensor<2, 3, 4>;
pub type Tensor3x3x3 = UsfTensor<3, 3, 3>;
pub type Tensor3x3x4 = UsfTensor<3, 3, 4>;
pub type Tensor3x4x4 = UsfTensor<3, 4, 4>;
pub type Tensor4x4x4 = UsfTensor<4, 4, 4>;
pub type Tensor2x4x8 = UsfTensor<2, 4, 8>;
pub type Tensor8x4x2 = UsfTensor<8, 4, 2>;

pub type UsfTensor2x2x2 = UsfTensor<2, 2, 2>;
pub type UsfTensor2x2x3 = UsfTensor<2, 2, 3>;
pub type UsfTensor2x3x3 = UsfTensor<2, 3, 3>;
pub type UsfTensor2x3x4 = UsfTensor<2, 3, 4>;
pub type UsfTensor3x3x3 = UsfTensor<3, 3, 3>;
pub type UsfTensor3x3x4 = UsfTensor<3, 3, 4>;
pub type UsfTensor3x4x4 = UsfTensor<3, 4, 4>;
pub type UsfTensor4x4x4 = UsfTensor<4, 4, 4>;
pub type UsfTensor2x4x8 = UsfTensor<2, 4, 8>;
pub type UsfTensor8x4x2 = UsfTensor<8, 4, 2>;

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorCoreOps<A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorFieldOps<A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorBridgeOps<A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorProjectionCoreOps<A, B, C> for UsfTensor<A, B, C> {}
