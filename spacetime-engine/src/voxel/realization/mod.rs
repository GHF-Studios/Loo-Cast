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
//!
//! ## Integration
//!
//! Generic spatial demand becomes voxel-specific realization intent here. The realization registry
//! resolves the disposable scale entity; materialization residency remains downstream.
//!
//! ## Module map
//!
//! - `contact`: Physical-boundary contact preparation and scope geometry.
//! - `demand`: Resolve semantic authority/Scale intent to disposable voxel scale realizations.
//! - `domain`: Voxel capability participation and local realization policy.
//! - `intent`: Translate generic spatial intent into voxel-specific capability scopes.
//! - `model`: Intent and resolved-demand snapshots across the voxel realization boundary.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::cmp::Reverse;

use bevy::prelude::*;

use crate::{
    ecs::UsfLogicalRealizationOf,
    spatial::{
        SpatialDemandMotionSnapshot, SpatialDemandScope, SpatialDemandSnapshot,
        SpatialRefinementDemand, SpatialScale, UsfPosition, UsfPrimaryInteractionSlice,
        UsfRefinementPlan, UsfResidencyRequests, UsfScaleLayer, UsfScaleRoleMask,
        UsfScaleSliceMask, UsfSemanticFrame, UsfViewDemand, UsfViewDemandSnapshot,
    },
};

use super::{
    CelestialVoxelField, CelestialVoxelRealizations, MATERIALIZATION_CHUNK_SIZE,
    VoxelMaterializationDemand, VoxelPinnedMaterializationDemand, VoxelScaleRealization,
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
pub(super) use demand::resolve_voxel_realization_demands;
pub use domain::VoxelScaleDomain;
pub(super) use intent::collect_voxel_realization_intents;
use model::{VoxelDemandSource, VoxelRealizationIntent, VoxelRealizationIntentTarget};
pub(in crate::voxel) use model::{
    VoxelRealizationDemandSnapshot, VoxelRealizationIntentSnapshot, VoxelRealizationScope,
    VoxelRealizationTarget,
};
