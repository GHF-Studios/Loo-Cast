//! Ancestor-closed runtime residency over canonical USF space.
//!
//! Canonical USF topology is virtual and always addressable through
//! [`UsfChunkAddress`]. Runtime movement does not create or destroy that
//! topology. This module tracks only the sparse subset of canonical contexts
//! that currently have runtime responsibility.
//!
//! Primary spatial interest is copied from [`SpatialDemandSnapshot`].
//! Capability planners may add derived requirements through
//! [`UsfResidencyRequestBuffer`]. The resulting [`UsfContextResidency`] is
//! ancestor-closed: a resident child always retains every canonical ancestor up
//! to the Scale-Slice ceiling.
//!
//! Residency is lifecycle/context fact, not realization authority. Voxel,
//! field, physics and render systems remain free to choose their own
//! capability-local representations beneath the resident contexts.

use super::{
    SPATIAL_SCALE_MAX, SpatialDemandScope, SpatialScale, UsfChunkAddress, UsfPosition,
    UsfPositionError,
};

mod model;
mod plan;
mod range;
mod systems;

pub use model::{UsfContextResidency, UsfResidencyRequestBuffer, UsfResidentContext};
pub use systems::UsfResidencySet;
pub(in crate::spatial) use systems::configure;

use model::UsfResidencyDemandRange;
use range::address_range_intersecting_demand;
