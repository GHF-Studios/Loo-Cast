#![allow(dead_code)]

use super::super::field::Field;
use super::super::matrix::usf::UsfMatrix;
use super::super::scalar::usf::UsfScalar;
use super::super::vector::usf::UsfVector;

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

impl<const A: usize, const B: usize, const C: usize> UsfTensor<A, B, C> {
    pub fn zero() -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if runtime validation rejects degenerate tensor shape constraints.
    pub fn from_slices(_slices: [UsfMatrix<B, C>; A]) -> Self {
        todo!()
    }
    pub fn to_slices(&self) -> [UsfMatrix<B, C>; A] {
        todo!()
    }
    pub fn add_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn sub_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if `rhs` is zero.
    pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn add(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    pub fn sub(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    pub fn mul(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any corresponding lane in `rhs` is zero.
    pub fn div(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any corresponding lane in `rhs` is zero.
    pub fn rem(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    pub fn min(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    pub fn max(&self, _rhs: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any lane has `lo > hi`.
    pub fn clamp(&self, _lo: UsfTensor<A, B, C>, _hi: UsfTensor<A, B, C>) -> Self {
        todo!()
    }
    pub fn get_dimensions(&self) -> (usize, usize, usize) {
        todo!()
    }
    pub fn get_element_count(&self) -> usize {
        todo!()
    }
    /// # Panics
    /// - Panics if `index` is out of bounds.
    pub fn get_slice(&self, _index: usize) -> UsfMatrix<B, C> {
        todo!()
    }
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if the target slice is immutable under runtime field mutability policy.
    pub fn set_slice(&mut self, _index: usize, _value: UsfMatrix<B, C>) {
        todo!()
    }
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    pub fn get_vector(&self, _i: usize, _j: usize) -> UsfVector<C> {
        todo!()
    }
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics if the target vector is immutable under runtime field mutability policy.
    pub fn set_vector(&mut self, _i: usize, _j: usize, _value: UsfVector<C>) {
        todo!()
    }
    /// # Panics
    /// - Panics if any index is out of bounds.
    pub fn get_lane(&self, _i: usize, _j: usize, _k: usize) -> UsfScalar {
        todo!()
    }
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics if the target lane is immutable under runtime field mutability policy.
    pub fn set_lane(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfScalar) {
        todo!()
    }
}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorCoreOps<UsfScalar, UsfMatrix<B, C>, UsfVector<C>, A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorFieldOps<UsfScalar, UsfMatrix<B, C>, UsfVector<C>, A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorBridgeOps<UsfScalar, UsfMatrix<B, C>, UsfVector<C>, A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize>
    super::shared::TensorProjectionCoreOps<UsfScalar, UsfMatrix<A, B>, UsfMatrix<A, C>, UsfMatrix<B, C>, UsfVector<A>, UsfVector<B>, UsfVector<C>, A, B, C>
    for UsfTensor<A, B, C>
{
}
