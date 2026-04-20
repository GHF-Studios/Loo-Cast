#![allow(dead_code)]

use crate::utils::one_of::OneOf2;

pub type UsfOrNormal<UsfT, NormalT> = OneOf2<UsfT, NormalT>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputDomain {
    Usf,
    Normal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputQualityConstraint {
    /// Require lossless precision/range preservation for requested output.
    RequireLossless,
    /// Permit precision/range loss when materializing requested output.
    AllowLossy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputMode {
    /// Requested output domain (USF or normal).
    pub domain: OutputDomain,
    /// Quality constraint used by runtime guard clauses.
    ///
    /// Runtime contract:
    /// - `OutputDomain::Usf + OutputQualityConstraint::AllowLossy` is rejected.
    /// - `OutputDomain::Normal + OutputQualityConstraint::RequireLossless` is accepted only for provably lossless projections.
    /// - `OutputDomain::Normal + OutputQualityConstraint::AllowLossy` enables lossy projection.
    pub quality_constraint: OutputQualityConstraint,
}
