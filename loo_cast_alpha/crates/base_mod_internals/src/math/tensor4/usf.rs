use super::super::field::Field;
use super::super::tensor::usf::UsfTensor;
pub use super::aliases::{Tensor4OrScalar, UsfOrNormalTensor4};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTensor4<const A: usize, const B: usize, const C: usize, const D: usize> {
    // CONTRACT: A,B,C,D >= 2. Any axis == 1 is reducible and forbidden.
    pub chunks: Field<[UsfTensor<B, C, D>; A]>,
}

pub type Tensor2x2x2x2 = UsfTensor4<2, 2, 2, 2>;
pub type Tensor2x2x3x4 = UsfTensor4<2, 2, 3, 4>;
pub type Tensor2x3x3x4 = UsfTensor4<2, 3, 3, 4>;
pub type Tensor3x3x3x3 = UsfTensor4<3, 3, 3, 3>;
pub type Tensor4x4x4x4 = UsfTensor4<4, 4, 4, 4>;
pub type Tensor2x4x4x8 = UsfTensor4<2, 4, 4, 8>;
pub type Tensor8x4x4x2 = UsfTensor4<8, 4, 4, 2>;

pub type UsfTensor2x2x2x2 = UsfTensor4<2, 2, 2, 2>;
pub type UsfTensor2x2x3x4 = UsfTensor4<2, 2, 3, 4>;
pub type UsfTensor2x3x3x4 = UsfTensor4<2, 3, 3, 4>;
pub type UsfTensor3x3x3x3 = UsfTensor4<3, 3, 3, 3>;
pub type UsfTensor4x4x4x4 = UsfTensor4<4, 4, 4, 4>;
pub type UsfTensor2x4x4x8 = UsfTensor4<2, 4, 4, 8>;
pub type UsfTensor8x4x4x2 = UsfTensor4<8, 4, 4, 2>;

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4CoreOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4FieldOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4BridgeOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4ProjectionCoreOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}
