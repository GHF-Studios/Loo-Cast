#![allow(dead_code)]

/// Mathematical kind targeted by an operation projection request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MathKind {
    Scalar,
    Vector,
    Matrix,
    Tensor3,
    Tensor4,
    Quaternion,
    Translation,
    Rotation,
    Scale,
    Transform,
}

/// Numeric representation regime requested for operation output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpRepr {
    Usf,
    Normal,
}

/// Generic operation projection configuration.
///
/// Intended for contract/facade layers where a call can project into
/// different mathematical kinds and representation regimes before
/// facade-level monomorphization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpMode {
    pub kind: MathKind,
    pub repr: OpRepr,
}

impl OpMode {
    #[inline]
    pub const fn usf(kind: MathKind) -> Self {
        Self {
            kind,
            repr: OpRepr::Usf,
        }
    }

    #[inline]
    pub const fn normal(kind: MathKind) -> Self {
        Self {
            kind,
            repr: OpRepr::Normal,
        }
    }
}
