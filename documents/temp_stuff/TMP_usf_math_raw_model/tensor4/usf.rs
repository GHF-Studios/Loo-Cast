#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::field::Field;
use super::super::matrix::aliases::UsfOrNormalMatrix;
use super::super::matrix::usf::UsfMatrix;
use super::super::scalar::aliases::UsfOrNormalScalar;
use super::super::scalar::usf::UsfScalar;
use super::super::tensor::aliases::UsfOrNormalTensor;
use super::super::tensor::usf::UsfTensor;
use super::super::vector::aliases::UsfOrNormalVector;
use super::super::vector::usf::UsfVector;
pub use super::aliases::{Tensor4OrScalar, UsfOrNormalTensor4};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTensor4<const A: usize, const B: usize, const C: usize, const D: usize> {
    // CONTRACT: A,B,C,D >= 2. Any axis == 1 is reducible and forbidden.
    pub(super) chunks: Field<[UsfTensor<B, C, D>; A]>,
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

impl<const A: usize, const B: usize, const C: usize, const D: usize> UsfTensor4<A, B, C, D> {
    /// Zero rank-4 tensor.
    pub fn zero() -> Self {
        todo!()
    }

    /// Builds rank-4 tensor from `A` chunks of shape `(B, C, D)`.
    /// # Domain
    /// - Accepts each chunk in `{Usf, Normal}` independently.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if runtime validation rejects degenerate tensor shape constraints.
    pub fn from_chunks(_chunks: [UsfOrNormalTensor<B, C, D>; A]) -> Self {
        todo!()
    }

    /// Returns axis-A chunks in requested output mode.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but chunk projection loses precision or range.
    pub fn to_chunks(&self, _output_mode: OutputMode) -> [UsfOrNormalTensor<B, C, D>; A] {
        todo!()
    }

    /// Adds tensor or scalar operand.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn add(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Subtracts tensor or scalar operand.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn sub(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Multiplies tensor or scalar operand component-wise.
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn component_mul(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
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
    pub fn component_div(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Returns element-wise minimum.
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn min(&self, _rhs: UsfOrNormalTensor4<A, B, C, D>) -> Self {
        todo!()
    }

    /// Returns element-wise maximum.
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    pub fn max(&self, _rhs: UsfOrNormalTensor4<A, B, C, D>) -> Self {
        todo!()
    }

    /// # Domain
    /// - Accepts all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any tensor component has `lo > hi`.
    pub fn clamp(&self, _lo: UsfOrNormalTensor4<A, B, C, D>, _hi: UsfOrNormalTensor4<A, B, C, D>) -> Self {
        todo!()
    }

    /// Returns `(A, B, C, D)` dimensions.
    pub fn get_dimensions(&self) -> (usize, usize, usize, usize) {
        todo!()
    }

    /// Returns total scalar component count.
    pub fn get_element_count(&self) -> usize {
        todo!()
    }

    /// Returns chunk orthogonal to axis A.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but chunk projection loses precision or range.
    pub fn get_chunk(&self, _index: usize, _output_mode: OutputMode) -> UsfOrNormalTensor<B, C, D> {
        todo!()
    }

    /// Sets chunk orthogonal to axis A.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target chunk is immutable under runtime field mutability policy.
    pub fn set_chunk(&mut self, _index: usize, _value: UsfOrNormalTensor<B, C, D>) {
        todo!()
    }

    /// Returns matrix at fixed `(A=i, B=j)` with shape `(C, D)`.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but matrix projection loses precision or range.
    pub fn get_matrix(&self, _i: usize, _j: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<C, D> {
        todo!()
    }

    /// Sets matrix at fixed `(A=i, B=j)` with shape `(C, D)`.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target matrix is immutable under runtime field mutability policy.
    pub fn set_matrix(&mut self, _i: usize, _j: usize, _value: UsfOrNormalMatrix<C, D>) {
        todo!()
    }

    /// Returns vector along axis D at fixed `(A=i, B=j, C=k)`.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `i`, `j`, or `k` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but vector projection loses precision or range.
    pub fn get_vector(&self, _i: usize, _j: usize, _k: usize, _output_mode: OutputMode) -> UsfOrNormalVector<D> {
        todo!()
    }

    /// Sets vector along axis D at fixed `(A=i, B=j, C=k)`.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `i`, `j`, or `k` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target vector is immutable under runtime field mutability policy.
    pub fn set_vector(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfOrNormalVector<D>) {
        todo!()
    }

    /// Returns scalar component at `(A=i, B=j, C=k, D=l)`.
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    pub fn get_component(&self, _i: usize, _j: usize, _k: usize, _l: usize, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets scalar component at `(A=i, B=j, C=k, D=l)`.
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target tensor component is immutable under runtime field mutability policy.
    pub fn set_component(&mut self, _i: usize, _j: usize, _k: usize, _l: usize, _value: UsfOrNormalScalar) {
        todo!()
    }
}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4CoreOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4FieldOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4BridgeOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4ProjectionCoreOps<A, B, C, D> for UsfTensor4<A, B, C, D> {}
