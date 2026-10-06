//! Multiscale voxel realization policy.
//!
//! Generic spatial interest says where gameplay currently cares about reality.
//! [`SpatialRefinementDemand`] says how fine nearby capability realization is
//! requested to become. This module translates those independent inputs into
//! voxel-specific realization scopes.
//!
//! The resulting scopes also contribute runtime-context residency requirements.
//! Residency does not choose the voxel plan; it records the canonical contexts
//! required by the plan.

use std::cmp::Reverse;

use bevy::prelude::*;

use crate::{
    ecs::UsfLogicalRealizationOf,
    spatial::{
        SpatialDemandMotionSnapshot, SpatialDemandScope, SpatialDemandSnapshot,
        SpatialRefinementDemand, SpatialScale, UsfChartMask, UsfPosition,
        UsfPrimaryInteractionSlice, UsfRefinementPlan, UsfResidencyRequestBuffer, UsfScaleLayer,
        UsfScaleRoleMask, UsfSemanticFrame,
    },
};

use super::{
    CelestialVoxelField, CelestialVoxelRealizationRegistry, MATERIALIZATION_CHUNK_SIZE,
    VoxelMaterializationDemand, VoxelPinnedDemand, VoxelWorld,
};

mod contact;
mod demand;
mod domain;
mod intent;
mod model;

use contact::{
    celestial_contact_volume_demand, materialization_residency_extent, observe_celestial_contact,
    priority_focus_within_scope, realization_plan,
};
pub(super) use demand::resolve_voxel_realization_demand;
pub use domain::VoxelScaleDomain;
pub(super) use intent::collect_voxel_realization_intent;
use model::{VoxelDemandSource, VoxelRealizationIntent, VoxelRealizationIntentTarget};
pub(in crate::voxel) use model::{
    VoxelRealizationDemandSnapshot, VoxelRealizationIntentSnapshot, VoxelRealizationScope,
    VoxelRealizationTarget,
};
