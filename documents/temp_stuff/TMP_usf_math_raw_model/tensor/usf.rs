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

impl<const A: usize, const B: usize, const C: usize> UsfTensor<A, B, C> {
    /// Zero tensor.
    pub fn zero() -> Self {
        todo!()
    }

    /// Builds tensor from `A` slices of shape `(B, C)`.
    /// # Domain
    /// - Accepts each slice in `{Usf, Normal}` independently.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if runtime validation rejects degenerate tensor shape constraints.
    pub fn from_slices(_slices: [UsfOrNormalMatrix<B, C>; A]) -> Self {
        todo!()
    }

    /// Returns axis-A slices in requested output mode.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but slice projection loses precision or range.
    pub fn to_slices(&self, _output_mode: OutputMode) -> [UsfOrNormalMatrix<B, C>; A] {
        todo!()
    }

    /// Adds tensor or scalar operand.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn add(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Subtracts tensor or scalar operand.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn sub(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Multiplies tensor or scalar operand component-wise.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn component_mul(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Divides tensor or scalar operand component-wise.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if divisor operand resolves to zero in any addressed tensor component.
    pub fn component_div(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Returns element-wise minimum.
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn min(&self, _rhs: UsfOrNormalTensor<A, B, C>) -> Self {
        todo!()
    }

    /// Returns element-wise maximum.
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn max(&self, _rhs: UsfOrNormalTensor<A, B, C>) -> Self {
        todo!()
    }

    /// # Domain
    /// - Accepts all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any tensor component has `lo > hi`.
    pub fn clamp(&self, _lo: UsfOrNormalTensor<A, B, C>, _hi: UsfOrNormalTensor<A, B, C>) -> Self {
        todo!()
    }

    /// Returns `(A, B, C)` dimensions.
    pub fn get_dimensions(&self) -> (usize, usize, usize) {
        todo!()
    }

    /// Returns total scalar component count.
    pub fn get_element_count(&self) -> usize {
        todo!()
    }

    /// Returns slice orthogonal to axis A.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but slice projection loses precision or range.
    pub fn get_slice(&self, _index: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<B, C> {
        todo!()
    }

    /// Sets slice orthogonal to axis A.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target slice is immutable under runtime field mutability policy.
    pub fn set_slice(&mut self, _index: usize, _value: UsfOrNormalMatrix<B, C>) {
        todo!()
    }

    /// Returns vector along axis C at fixed `(A=i, B=j)`.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but vector projection loses precision or range.
    pub fn get_vector(&self, _i: usize, _j: usize, _output_mode: OutputMode) -> UsfOrNormalVector<C> {
        todo!()
    }

    /// Sets vector along axis C at fixed `(A=i, B=j)`.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target vector is immutable under runtime field mutability policy.
    pub fn set_vector(&mut self, _i: usize, _j: usize, _value: UsfOrNormalVector<C>) {
        todo!()
    }

    /// Returns scalar component at `(A=i, B=j, C=k)`.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    pub fn get_component(&self, _i: usize, _j: usize, _k: usize, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets scalar component at `(A=i, B=j, C=k)`.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target tensor component is immutable under runtime field mutability policy.
    pub fn set_component(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfOrNormalScalar) {
        todo!()
    }
}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorCoreOps<A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorFieldOps<A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorBridgeOps<A, B, C> for UsfTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorProjectionCoreOps<A, B, C> for UsfTensor<A, B, C> {}
