//! Generic scale-local capability realization and realized coverage.
//!
//! Demand is intent. Residency is canonical context responsibility.
//! [`UsfCapabilityRealization`] is live capability-local state.
//! [`UsfScaleCoverage`] is the bounded realized fact derived from that state.
//!
//! Coverage is persistent across frames until the underlying realization changes
//! or retires. It is never cleared speculatively before capability planners read
//! it.
//!
//! ## Module map
//!
//! - `model`: Live capability facts and bounded realized coverage records.
//! - `roles`: Scale-local capability role vocabulary.
//! - `snapshot`: Persistent index of realized capability facts.
//! - `systems`: ECS publication of the persistent coverage snapshot.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::collections::HashMap;

use bevy::prelude::*;

use super::{SpatialScale, UsfChunkAddress, UsfPosition, UsfSpatialSet};

mod model;
mod roles;
mod snapshot;
mod systems;

pub use model::{
    UsfCapabilityCoverageFact, UsfCapabilityCoveragePublication, UsfCapabilityRealization,
    UsfRefinementAperture, UsfScaleCoverage,
};
pub use roles::UsfScaleRoleMask;
pub use snapshot::UsfScaleCoverageSnapshot;
pub use systems::UsfCapabilitySet;
pub(in crate::spatial) use systems::configure;
