use super::super::matrix::usf::UsfMatrix;
pub use super::aliases::{TensorOrScalar, UsfOrNormalTensor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTensor<const A: usize, const B: usize, const C: usize> {
    // CONTRACT: A,B,C >= 2. Any axis == 1 is reducible and forbidden.
    pub slices: [UsfMatrix<B, C>; A],
}
impl<const A: usize, const B: usize, const C: usize> super::shared::TensorCoreOps<A, B, C> for UsfTensor<A, B, C> {}
impl<const A: usize, const B: usize, const C: usize> super::shared::TensorFieldOps<A, B, C> for UsfTensor<A, B, C> {}
impl<const A: usize, const B: usize, const C: usize> super::shared::TensorBridgeOps<A, B, C> for UsfTensor<A, B, C> {}
impl<const A: usize, const B: usize, const C: usize> super::shared::TensorProjectionCoreOps<A, B, C> for UsfTensor<A, B, C> {}

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
