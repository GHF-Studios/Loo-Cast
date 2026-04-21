#![allow(dead_code)]

use crate::utils::one_of::OneOf2;

/// Domain union helper used across math contracts.
pub type UsfOrNormal<UsfT, NormalT> = OneOf2<UsfT, NormalT>;

/// Output representation family requested by projection-style operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputDomain {
    Usf,
    Normal,
}

/// Output projection quality policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputQualityConstraint {
    /// Require lossless precision/range preservation for requested output.
    RequireLossless,
    /// Permit precision/range loss when materializing requested output.
    AllowLossy,
}

/// Runtime output projection configuration used by mixed-domain operations.
/// Intended usage:
/// - Keep this explicit in APIs that can project to either USF or normal output.
/// - Let facade layers provide ergonomic constructors and script-syntax wrappers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputMode {
    /// Requested output domain (USF or normal).
    pub domain: OutputDomain,
    /// Quality constraint used by runtime guard clauses.
    /// Runtime contract:
    /// - `OutputDomain::Usf + OutputQualityConstraint::AllowLossy` is rejected.
    /// - `OutputDomain::Normal + OutputQualityConstraint::RequireLossless` is accepted only for provably lossless projections.
    /// - `OutputDomain::Normal + OutputQualityConstraint::AllowLossy` enables lossy projection.
    pub quality_constraint: OutputQualityConstraint,
}
