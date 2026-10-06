//! Semantic long-distance navigation.
//!
//! Travel influences describe structure without becoming colliders or presentation
//! LODs. Observer context selects a useful movement scale from that structure.

use super::{SpatialScale, UsfPosition, UsfSemanticFrame};

mod context;
mod travel;

pub use context::{UsfApproachRefinement, UsfNavigationContext, UsfNavigationContextKind};
pub use travel::{
    UsfTravelBoundary, UsfTravelBoundaryResolver, UsfTravelBoundarySample,
    UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelInfluenceMeasure,
    UsfTravelMedium, UsfTravelNeighborhood,
};
