//! Semantic long-distance navigation.
//!
//! Travel influences describe structure without becoming colliders or presentation
//! LODs. Observer context selects a useful movement scale from that structure.
//!
//! ## Module map
//!
//! - `context`: Observer-local navigation context and refinement capability.
//! - `travel`: Semantic travel structure, measurement and observer-local selection.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use super::{SpatialScale, UsfPosition, UsfSemanticFrame};

mod context;
mod travel;

pub use context::{UsfApproachRefinement, UsfNavigationContext, UsfNavigationContextKind};
pub use travel::{
    UsfTravelBoundary, UsfTravelBoundaryProvider, UsfTravelBoundarySample, UsfTravelInfluence,
    UsfTravelInfluenceKind, UsfTravelInfluenceMeasure, UsfTravelMedium, UsfTravelNeighborhood,
};
