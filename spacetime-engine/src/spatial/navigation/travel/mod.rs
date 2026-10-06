//! Semantic travel structure, measurement and observer-local selection.

use super::{SpatialScale, UsfPosition, UsfSemanticFrame};

mod boundary;
mod influence;
mod neighborhood;

pub use boundary::{UsfTravelBoundary, UsfTravelBoundaryResolver, UsfTravelBoundarySample};
pub use influence::{
    UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelInfluenceMeasure, UsfTravelMedium,
};
pub use neighborhood::UsfTravelNeighborhood;
