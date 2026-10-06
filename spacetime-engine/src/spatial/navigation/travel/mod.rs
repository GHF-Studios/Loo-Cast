//! Semantic travel structure, measurement and observer-local selection.
//!
//! ## Module map
//!
//! - `boundary`: Refinable hard-body surface provider; navigation samples but does not own it.
//! - `influence`: Coarse semantic extent and bounded observer measurement.
//! - `neighborhood`: Observer-local cache and selection policy for nearby travel influences.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use super::{SpatialScale, UsfPosition, UsfSemanticFrame};

mod boundary;
mod influence;
mod neighborhood;

pub use boundary::{UsfTravelBoundary, UsfTravelBoundaryProvider, UsfTravelBoundarySample};
pub use influence::{
    UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelInfluenceMeasure, UsfTravelMedium,
};
pub use neighborhood::UsfTravelNeighborhood;
