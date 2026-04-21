#![allow(dead_code)]

//! Shared rank-4 tensor contracts for USF/normal tensor surfaces.
//!
//! Facade-first rule:
//! - This layer defines semantics and panic behavior.
//! - Script-facing operations should be exposed by concrete facades and bindings.
//!
//! Domain/quality mechanism:
//! - Mixed-domain operands use `UsfOrNormal*` aliases and `Tensor4OrScalar`.
//! - Output projection requests are expressed through `OutputMode`.
//! - Invalid domain-quality combinations panic fast.

use super::super::aliases::OutputMode;
use super::super::matrix::aliases::UsfOrNormalMatrix;
use super::super::scalar::aliases::UsfOrNormalScalar;
use super::super::tensor::aliases::UsfOrNormalTensor;
use super::super::vector::aliases::UsfOrNormalVector;
use super::aliases::{Tensor4OrScalar, UsfOrNormalTensor4};

/// Rank-4 tensor core operations.
/// # Working Principle
/// - `A`, `B`, `C`, and `D` are compile-time axis extents for this rank-4 tensor contract.
/// - Core methods preserve rank/shape semantics while exposing explicit projection points.
/// # Usage
/// - Implement this trait on concrete rank-4 tensor carriers.
/// - Use `Tensor4Contract<A, B, C, D>` bounds in generic consumers.
pub trait Tensor4CoreOps<const A: usize, const B: usize, const C: usize, const D: usize>: Clone + Sized {
    /// Zero rank-4 tensor.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn zero() -> Self {
        todo!()
    }

    /// Builds rank-4 tensor from `A` chunks of shape `(B, C, D)`.
    ///
    /// # Parameters
    /// - `chunks` ([UsfOrNormalTensor<B, C, D>; A]): Tensor chunks used to build the rank-4 tensor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts each chunk in `{Usf, Normal}` independently.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if runtime validation rejects degenerate tensor shape constraints.
    fn from_chunks(_chunks: [UsfOrNormalTensor<B, C, D>; A]) -> Self {
        todo!()
    }

    /// Returns axis-A chunks in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[UsfOrNormalTensor<B, C, D>; A]`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but chunk projection loses precision or range.
    fn to_chunks(&self, _output_mode: OutputMode) -> [UsfOrNormalTensor<B, C, D>; A] {
        todo!()
    }

    /// Adds tensor or scalar operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Tensor4OrScalar<A, B, C, D>): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn add(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Subtracts tensor or scalar operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Tensor4OrScalar<A, B, C, D>): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn sub(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Multiplies tensor or scalar operand component-wise.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Tensor4OrScalar<A, B, C, D>): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn component_mul(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Divides tensor or scalar operand component-wise.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Tensor4OrScalar<A, B, C, D>): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts tensor branch with `{self: Usf, rhs_tensor: Usf}` and `{self: Usf, rhs_tensor: Normal}`.
    /// - Accepts scalar branch with `{self: Usf, rhs_scalar: Usf}` and `{self: Usf, rhs_scalar: Normal}`.
    /// - Disallowed combinations: passing both tensor and scalar operands in the same call, because `OneOf2` selects exactly one branch.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if divisor operand resolves to zero in any addressed tensor component.
    fn component_div(&self, _rhs: Tensor4OrScalar<A, B, C, D>) -> Self {
        todo!()
    }

    /// Returns element-wise minimum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalTensor4<A, B, C, D>): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn min(&self, _rhs: UsfOrNormalTensor4<A, B, C, D>) -> Self {
        todo!()
    }

    /// Returns element-wise maximum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalTensor4<A, B, C, D>): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn max(&self, _rhs: UsfOrNormalTensor4<A, B, C, D>) -> Self {
        todo!()
    }

    /// Clamps the value to the provided bounds.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `lo` (UsfOrNormalTensor4<A, B, C, D>): Lower bound.
    /// - `hi` (UsfOrNormalTensor4<A, B, C, D>): Upper bound.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any tensor component has `lo > hi`.
    fn clamp(&self, _lo: UsfOrNormalTensor4<A, B, C, D>, _hi: UsfOrNormalTensor4<A, B, C, D>) -> Self {
        todo!()
    }

    /// Returns `(A, B, C, D)` dimensions.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Tuple result of type `(usize, usize, usize, usize)`.
    fn get_dimensions(&self) -> (usize, usize, usize, usize) {
        todo!()
    }

    /// Returns total scalar component count.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Numeric metadata value (usize).
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

/// Rank-4 tensor field access contract.
pub trait Tensor4FieldOps<const A: usize, const B: usize, const C: usize, const D: usize>: Tensor4CoreOps<A, B, C, D> {
    /// Returns chunk orthogonal to axis A.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `index` (usize): Zero-based index.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalTensor<B, C, D>`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but chunk projection loses precision or range.
    fn get_chunk(&self, _index: usize, _output_mode: OutputMode) -> UsfOrNormalTensor<B, C, D> {
        todo!()
    }

    /// Sets chunk orthogonal to axis A.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `index` (usize): Zero-based index.
    /// - `value` (UsfOrNormalTensor<B, C, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target chunk is immutable under runtime field mutability policy.
    fn set_chunk(&mut self, _index: usize, _value: UsfOrNormalTensor<B, C, D>) {
        todo!()
    }

    /// Returns matrix at fixed `(A=i, B=j)` with shape `(C, D)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `i` (usize): Axis index i (zero-based).
    /// - `j` (usize): Axis index j (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<C, D>`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but matrix projection loses precision or range.
    fn get_matrix(&self, _i: usize, _j: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<C, D> {
        todo!()
    }

    /// Sets matrix at fixed `(A=i, B=j)` with shape `(C, D)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `i` (usize): Axis index i (zero-based).
    /// - `j` (usize): Axis index j (zero-based).
    /// - `value` (UsfOrNormalMatrix<C, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `i` or `j` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target matrix is immutable under runtime field mutability policy.
    fn set_matrix(&mut self, _i: usize, _j: usize, _value: UsfOrNormalMatrix<C, D>) {
        todo!()
    }

    /// Returns vector along axis D at fixed `(A=i, B=j, C=k)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `i` (usize): Axis index i (zero-based).
    /// - `j` (usize): Axis index j (zero-based).
    /// - `k` (usize): Axis index k (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<D>`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `i`, `j`, or `k` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but vector projection loses precision or range.
    fn get_vector(&self, _i: usize, _j: usize, _k: usize, _output_mode: OutputMode) -> UsfOrNormalVector<D> {
        todo!()
    }

    /// Sets vector along axis D at fixed `(A=i, B=j, C=k)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `i` (usize): Axis index i (zero-based).
    /// - `j` (usize): Axis index j (zero-based).
    /// - `k` (usize): Axis index k (zero-based).
    /// - `value` (UsfOrNormalVector<D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `i`, `j`, or `k` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target vector is immutable under runtime field mutability policy.
    fn set_vector(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfOrNormalVector<D>) {
        todo!()
    }

    /// Returns scalar component at `(A=i, B=j, C=k, D=l)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `i` (usize): Axis index i (zero-based).
    /// - `j` (usize): Axis index j (zero-based).
    /// - `k` (usize): Axis index k (zero-based).
    /// - `l` (usize): Axis index l (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_component(&self, _i: usize, _j: usize, _k: usize, _l: usize, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets scalar component at `(A=i, B=j, C=k, D=l)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `i` (usize): Axis index i (zero-based).
    /// - `j` (usize): Axis index j (zero-based).
    /// - `k` (usize): Axis index k (zero-based).
    /// - `l` (usize): Axis index l (zero-based).
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if any index is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the target tensor component is immutable under runtime field mutability policy.
    fn set_component(&mut self, _i: usize, _j: usize, _k: usize, _l: usize, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// Higher-rank projection helpers for rank-4 tensors.
/// # Panics
/// - Getter methods panic if any provided index is out of bounds.
/// - Getter methods panic when `output_mode` requests an unsupported projection policy.
/// - Setter methods panic if any provided index is out of bounds.
/// - Setter methods panic if domain combination is invalid for this operation.
/// - Setter methods panic when the addressed field is immutable under backend field policy.
pub trait Tensor4ProjectionCoreOps<const A: usize, const B: usize, const C: usize, const D: usize>: Tensor4FieldOps<A, B, C, D> {
    /// Returns tensor view orthogonal to axis A.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalTensor<B, C, D>`.
    fn get_tensor_bcd(&self, _a: usize, _output_mode: OutputMode) -> UsfOrNormalTensor<B, C, D> {
        todo!()
    }

    /// Sets tensor view orthogonal to axis A.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `value` (UsfOrNormalTensor<B, C, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_tensor_bcd(&mut self, _a: usize, _value: UsfOrNormalTensor<B, C, D>) {
        todo!()
    }

    /// Returns tensor view orthogonal to axis B.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalTensor<A, C, D>`.
    fn get_tensor_acd(&self, _b: usize, _output_mode: OutputMode) -> UsfOrNormalTensor<A, C, D> {
        todo!()
    }

    /// Sets tensor view orthogonal to axis B.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `value` (UsfOrNormalTensor<A, C, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_tensor_acd(&mut self, _b: usize, _value: UsfOrNormalTensor<A, C, D>) {
        todo!()
    }

    /// Returns tensor view orthogonal to axis C.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalTensor<A, B, D>`.
    fn get_tensor_abd(&self, _c: usize, _output_mode: OutputMode) -> UsfOrNormalTensor<A, B, D> {
        todo!()
    }

    /// Sets tensor view orthogonal to axis C.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `value` (UsfOrNormalTensor<A, B, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_tensor_abd(&mut self, _c: usize, _value: UsfOrNormalTensor<A, B, D>) {
        todo!()
    }

    /// Returns tensor view orthogonal to axis D.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalTensor<A, B, C>`.
    fn get_tensor_abc(&self, _d: usize, _output_mode: OutputMode) -> UsfOrNormalTensor<A, B, C> {
        todo!()
    }

    /// Sets tensor view orthogonal to axis D.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalTensor<A, B, C>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_tensor_abc(&mut self, _d: usize, _value: UsfOrNormalTensor<A, B, C>) {
        todo!()
    }

    /// Returns `(C, D)` matrix at fixed `(A=a, B=b)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<C, D>`.
    fn get_matrix_cd(&self, _a: usize, _b: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<C, D> {
        todo!()
    }

    /// Sets `(C, D)` matrix at fixed `(A=a, B=b)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `value` (UsfOrNormalMatrix<C, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_matrix_cd(&mut self, _a: usize, _b: usize, _value: UsfOrNormalMatrix<C, D>) {
        todo!()
    }

    /// Returns `(B, D)` matrix at fixed `(A=a, C=c)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<B, D>`.
    fn get_matrix_bd(&self, _a: usize, _c: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<B, D> {
        todo!()
    }

    /// Sets `(B, D)` matrix at fixed `(A=a, C=c)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `value` (UsfOrNormalMatrix<B, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_matrix_bd(&mut self, _a: usize, _c: usize, _value: UsfOrNormalMatrix<B, D>) {
        todo!()
    }

    /// Returns `(B, C)` matrix at fixed `(A=a, D=d)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<B, C>`.
    fn get_matrix_bc(&self, _a: usize, _d: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<B, C> {
        todo!()
    }

    /// Sets `(B, C)` matrix at fixed `(A=a, D=d)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalMatrix<B, C>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_matrix_bc(&mut self, _a: usize, _d: usize, _value: UsfOrNormalMatrix<B, C>) {
        todo!()
    }

    /// Returns `(A, D)` matrix at fixed `(B=b, C=c)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<A, D>`.
    fn get_matrix_ad(&self, _b: usize, _c: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<A, D> {
        todo!()
    }

    /// Sets `(A, D)` matrix at fixed `(B=b, C=c)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `value` (UsfOrNormalMatrix<A, D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_matrix_ad(&mut self, _b: usize, _c: usize, _value: UsfOrNormalMatrix<A, D>) {
        todo!()
    }

    /// Returns `(A, C)` matrix at fixed `(B=b, D=d)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<A, C>`.
    fn get_matrix_ac(&self, _b: usize, _d: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<A, C> {
        todo!()
    }

    /// Sets `(A, C)` matrix at fixed `(B=b, D=d)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalMatrix<A, C>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_matrix_ac(&mut self, _b: usize, _d: usize, _value: UsfOrNormalMatrix<A, C>) {
        todo!()
    }

    /// Returns `(A, B)` matrix at fixed `(C=c, D=d)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMatrix<A, B>`.
    fn get_matrix_ab(&self, _c: usize, _d: usize, _output_mode: OutputMode) -> UsfOrNormalMatrix<A, B> {
        todo!()
    }

    /// Sets `(A, B)` matrix at fixed `(C=c, D=d)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalMatrix<A, B>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_matrix_ab(&mut self, _c: usize, _d: usize, _value: UsfOrNormalMatrix<A, B>) {
        todo!()
    }

    /// Returns vector along axis D.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<D>`.
    fn get_vector_d(&self, _a: usize, _b: usize, _c: usize, _output_mode: OutputMode) -> UsfOrNormalVector<D> {
        todo!()
    }

    /// Sets vector along axis D.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `value` (UsfOrNormalVector<D>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_vector_d(&mut self, _a: usize, _b: usize, _c: usize, _value: UsfOrNormalVector<D>) {
        todo!()
    }

    /// Returns vector along axis C.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<C>`.
    fn get_vector_c(&self, _a: usize, _b: usize, _d: usize, _output_mode: OutputMode) -> UsfOrNormalVector<C> {
        todo!()
    }

    /// Sets vector along axis C.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalVector<C>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_vector_c(&mut self, _a: usize, _b: usize, _d: usize, _value: UsfOrNormalVector<C>) {
        todo!()
    }

    /// Returns vector along axis B.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<B>`.
    fn get_vector_b(&self, _a: usize, _c: usize, _d: usize, _output_mode: OutputMode) -> UsfOrNormalVector<B> {
        todo!()
    }

    /// Sets vector along axis B.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `a` (usize): Axis index a (zero-based).
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalVector<B>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_vector_b(&mut self, _a: usize, _c: usize, _d: usize, _value: UsfOrNormalVector<B>) {
        todo!()
    }

    /// Returns vector along axis A.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<A>`.
    fn get_vector_a(&self, _b: usize, _c: usize, _d: usize, _output_mode: OutputMode) -> UsfOrNormalVector<A> {
        todo!()
    }

    /// Sets vector along axis A.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (usize): Secondary operand used by the operation.
    /// - `c` (usize): Tertiary operand used by the operation.
    /// - `d` (usize): Axis index d (zero-based).
    /// - `value` (UsfOrNormalVector<A>): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_vector_a(&mut self, _b: usize, _c: usize, _d: usize, _value: UsfOrNormalVector<A>) {
        todo!()
    }
}

/// Bridge-only extension point for rank-4 tensor surfaces.
pub trait Tensor4BridgeOps<const A: usize, const B: usize, const C: usize, const D: usize>: Tensor4CoreOps<A, B, C, D> {}

/// Full rank-4 tensor contract.
pub trait Tensor4Contract<const A: usize, const B: usize, const C: usize, const D: usize>:
    Tensor4CoreOps<A, B, C, D> + Tensor4FieldOps<A, B, C, D> + Tensor4BridgeOps<A, B, C, D>
{
}
impl<T, const A: usize, const B: usize, const C: usize, const D: usize> Tensor4Contract<A, B, C, D> for T where
    T: Tensor4CoreOps<A, B, C, D> + Tensor4FieldOps<A, B, C, D> + Tensor4BridgeOps<A, B, C, D>
{
}
