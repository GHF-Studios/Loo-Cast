//! Generic scale-local capability realization and realized coverage.
//!
//! Demand is intent. Residency is canonical context responsibility.
//! [`UsfCapabilityRealization`] is live capability-local state.
//! [`UsfScaleCoverage`] is the bounded realized fact derived from that state.
//!
//! Coverage is persistent across frames until the underlying realization changes
//! or retires. It is never cleared speculatively before capability planners read
//! it.

use std::collections::HashMap;

use bevy::prelude::*;

use super::{SpatialScale, UsfChunkAddress, UsfPosition, UsfSpatialSet};

mod model;
mod roles;
mod snapshot;
mod systems;

pub use model::{
    UsfCapabilityCoverageBatch, UsfCapabilityCoverageRecord, UsfCapabilityRealization,
    UsfRefinementAperture, UsfScaleCoverage,
};
pub use roles::UsfScaleRoleMask;
pub use snapshot::UsfScaleCoverageSnapshot;
pub use systems::UsfCapabilitySet;
pub(in crate::spatial) use systems::configure;
