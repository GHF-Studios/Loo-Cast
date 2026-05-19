use core::marker::PhantomData;

/// Type-level marker for mathematical shape families.
pub trait MathKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScalarKind;
impl MathKind for ScalarKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VectorKind;
impl MathKind for VectorKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MatrixKind;
impl MathKind for MatrixKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tensor3Kind;
impl MathKind for Tensor3Kind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tensor4Kind;
impl MathKind for Tensor4Kind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QuaternionKind;
impl MathKind for QuaternionKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TranslationKind;
impl MathKind for TranslationKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RotationKind;
impl MathKind for RotationKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScaleKind;
impl MathKind for ScaleKind {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransformKind;
impl MathKind for TransformKind {}

/// Type-level marker for representation regime.
pub trait OpRepr {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsfRepr;
impl OpRepr for UsfRepr {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NormalRepr;
impl OpRepr for NormalRepr {}

/// Type-level OpMode contract.
///
/// OpMode is not a runtime configuration value.
/// It exists only as a generic parameterization (`Kind`, `Repr`) for
/// facade monomorphization and overload selection.
pub trait OpMode {
    type Kind: MathKind;
    type Repr: OpRepr;
}

/// Generic type-level op-mode marker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mode<K: MathKind, R: OpRepr>(PhantomData<(K, R)>);

impl<K: MathKind, R: OpRepr> OpMode for Mode<K, R> {
    type Kind = K;
    type Repr = R;
}
