/// Semantic multiplication-operation kinds for vector carriers.
///
/// These tags distinguish mathematically distinct operations that can share
/// syntactic multiplication-like notation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorMulKind {
    /// Vector scaled by scalar (`V × S -> V`).
    Scale,
    /// Component-wise vector product (`V × V -> V`).
    ComponentMul,
    /// Dot product (`V × V -> S`).
    Dot,
    /// Cross product (`V × V -> V`, only valid for `D == 3`).
    Cross,
}
