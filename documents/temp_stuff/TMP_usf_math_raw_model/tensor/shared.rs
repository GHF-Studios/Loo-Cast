#![allow(dead_code)]

//! Shared rank-3 tensor contracts for USF/normal tensor surfaces.
//!
//! Facade-first rule:
//! - Traits here specify operation semantics, not direct script API layout.
//! - Rhai-facing APIs should be generated from concrete facades.
//!
//! Domain/quality mechanism:
//! - Mixed-domain operands use `UsfOrNormal*` aliases and `TensorOrScalar`.
//! - Output projection selection uses `OutputMode`.
//! - Unsupported domain/quality combinations panic fast.

use super::super::aliases::OutputMode;
use super::super::matrix::aliases::UsfOrNormalMatrix;
use super::super::scalar::aliases::UsfOrNormalScalar;
use super::super::vector::aliases::UsfOrNormalVector;
use super::aliases::{TensorOrScalar, UsfOrNormalTensor};

/// Rank-3 tensor core operations.
/// Rhai surface target:
/// - Bind selected shape aliases as explicit overload groups.
/// - Keep axis-projection method names stable for script ergonomics.
pub trait TensorCoreOps<const A: usize, const B: usize, const C: usize>: Clone + Sized {
    /// Zero tensor.
    fn zero() -> Self {
        todo!()
    }

    /// Builds tensor from axis-A slices.
    /// `A` slices are expected, each with shape `(B, C)`.
    /// # Domain
    /// - Accepts mixed-domain slice values (`Usf` and `Normal`) within the same tensor payload.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if runtime shape/constraint validation fails.
    fn from_slices(_slices: [UsfOrNormalMatrix<B, C>; A]) -> Self {
        todo!()
    }

    /// Returns axis-A slices in requested output mode.
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn to_slices(&self, _output_mode: OutputMode) -> [UsfOrNormalMatrix<B, C>; A] {
        todo!()
    }

    /// Adds tensor-or-scalar operand component-wise.
    /// # Domain
    /// - Tensor branch and scalar branch are represented via `TensorOrScalar`.
    /// - Accepts mixed-domain operands where backend policy allows it.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn add(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Subtracts tensor-or-scalar operand component-wise.
    /// # Domain
    /// - Tensor branch and scalar branch are represented via `TensorOrScalar`.
    /// - Accepts mixed-domain operands where backend policy allows it.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn sub(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Multiplies tensor or scalar operand component-wise.
    /// # Domain
    /// - Tensor branch and scalar branch are represented via `TensorOrScalar`.
    /// - Accepts mixed-domain operands where backend policy allows it.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn component_mul(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Divides tensor or scalar operand component-wise.
    /// # Domain
    /// - Tensor branch and scalar branch are represented via `TensorOrScalar`.
    /// - Accepts mixed-domain operands where backend policy allows it.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any addressed divisor component resolves to zero.
    fn component_div(&self, _rhs: TensorOrScalar<A, B, C>) -> Self {
        todo!()
    }

    /// Returns element-wise minimum.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn min(&self, _rhs: UsfOrNormalTensor<A, B, C>) -> Self {
        todo!()
    }

    /// Returns element-wise maximum.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn max(&self, _rhs: UsfOrNormalTensor<A, B, C>) -> Self {
        todo!()
    }

    /// Clamps element-wise to `[lo, hi]`.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any addressed component has `lo > hi`.
    fn clamp(&self, _lo: UsfOrNormalTensor<A, B, C>, _hi: UsfOrNormalTensor<A, B, C>) -> Self {
        todo!()
    }

    /// Returns `(A, B, C)` dimensions.
    fn get_dimensions(&self) -> (usize, usize, usize) {
        todo!()
    }

    /// Returns total element count.
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

/// Rank-3 tensor field access contract.
pub trait TensorFieldOps<const A: usize, const B: usize, const C: usize>: TensorCoreOps<A, B, C> {
    /// Slice orthogonal to axis A, shape `(B, C)`.
    /// # Panics
    /// - Panics if `_index` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_slice(&self, _index: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<B, C> {
        todo!()
    }

    /// Slice orthogonal to axis A, shape `(B, C)`.
    /// # Panics
    /// - Panics if `_index` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_slice(&mut self, _index: usize, _value: UsfOrNormalMatrix<B, C>) {
        todo!()
    }

    /// Vector along axis C at fixed `(A=i, B=j)`.
    /// # Panics
    /// - Panics if `(_i, _j)` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_vector(&self, _i: usize, _j: usize, _output_mode: OutputMode) -> UsfOrNormalVector<C> {
        todo!()
    }

    /// Vector along axis C at fixed `(A=i, B=j)`.
    /// # Panics
    /// - Panics if `(_i, _j)` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_vector(&mut self, _i: usize, _j: usize, _value: UsfOrNormalVector<C>) {
        todo!()
    }

    /// Returns scalar tensor component `(i, j, k)`.
    /// # Panics
    /// - Panics if `(_i, _j, _k)` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_component(&self, _i: usize, _j: usize, _k: usize, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets scalar tensor component `(i, j, k)`.
    /// # Panics
    /// - Panics if `(_i, _j, _k)` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_component(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// Rank-3 tensor projection helpers across alternate axis selections.
pub trait TensorProjectionCoreOps<const A: usize, const B: usize, const C: usize>: TensorFieldOps<A, B, C> {
    /// Slice orthogonal to axis C, shape `(A, B)`.
    /// # Panics
    /// - Panics if `_k` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_matrix_ab(&self, _k: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<A, B> {
        todo!()
    }

    /// Slice orthogonal to axis C, shape `(A, B)`.
    /// # Panics
    /// - Panics if `_k` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_matrix_ab(&mut self, _k: usize, _value: UsfOrNormalMatrix<A, B>) {
        todo!()
    }

    /// Slice orthogonal to axis B, shape `(A, C)`.
    /// # Panics
    /// - Panics if `_j` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_matrix_ac(&self, _j: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<A, C> {
        todo!()
    }

    /// Slice orthogonal to axis B, shape `(A, C)`.
    /// # Panics
    /// - Panics if `_j` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_matrix_ac(&mut self, _j: usize, _value: UsfOrNormalMatrix<A, C>) {
        todo!()
    }

    /// Slice orthogonal to axis A, shape `(B, C)`.
    /// # Panics
    /// - Panics if `_i` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_matrix_bc(&self, _i: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<B, C> {
        todo!()
    }

    /// Slice orthogonal to axis A, shape `(B, C)`.
    /// # Panics
    /// - Panics if `_i` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_matrix_bc(&mut self, _i: usize, _value: UsfOrNormalMatrix<B, C>) {
        todo!()
    }

    /// Vector along axis A at fixed `(B=b, C=c)`.
    /// # Panics
    /// - Panics if `(_b, _c)` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_vector_a(&self, _b: usize, _c: usize, _output_mode: OutputMode) -> UsfOrNormalVector<A> {
        todo!()
    }

    /// Vector along axis A at fixed `(B=b, C=c)`.
    /// # Panics
    /// - Panics if `(_b, _c)` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_vector_a(&mut self, _b: usize, _c: usize, _value: UsfOrNormalVector<A>) {
        todo!()
    }

    /// Vector along axis B at fixed `(A=a, C=c)`.
    /// # Panics
    /// - Panics if `(_a, _c)` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_vector_b(&self, _a: usize, _c: usize, _output_mode: OutputMode) -> UsfOrNormalVector<B> {
        todo!()
    }

    /// Vector along axis B at fixed `(A=a, C=c)`.
    /// # Panics
    /// - Panics if `(_a, _c)` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_vector_b(&mut self, _a: usize, _c: usize, _value: UsfOrNormalVector<B>) {
        todo!()
    }

    /// Vector along axis C at fixed `(A=a, B=b)`.
    /// # Panics
    /// - Panics if `(_a, _b)` is out of bounds.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn get_vector_c(&self, _a: usize, _b: usize, _output_mode: OutputMode) -> UsfOrNormalVector<C> {
        todo!()
    }

    /// Vector along axis C at fixed `(A=a, B=b)`.
    /// # Panics
    /// - Panics if `(_a, _b)` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when the addressed field is immutable under backend field policy.
    fn set_vector_c(&mut self, _a: usize, _b: usize, _value: UsfOrNormalVector<C>) {
        todo!()
    }
}

/// Bridge-only extension point for rank-3 tensor surfaces.
pub trait TensorBridgeOps<const A: usize, const B: usize, const C: usize>: TensorCoreOps<A, B, C> {}

/// Full rank-3 tensor contract.
pub trait TensorContract<const A: usize, const B: usize, const C: usize>: TensorCoreOps<A, B, C> + TensorFieldOps<A, B, C> + TensorBridgeOps<A, B, C> {}
impl<T, const A: usize, const B: usize, const C: usize> TensorContract<A, B, C> for T where
    T: TensorCoreOps<A, B, C> + TensorFieldOps<A, B, C> + TensorBridgeOps<A, B, C>
{
}
