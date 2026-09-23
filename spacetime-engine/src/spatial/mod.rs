//! USF semantic spatial identity projected into bounded local runtime coordinates.
//!
//! Canonical positions remain independent from bounded local runtime charts.
//! All 71 Scale Slices from S-35 through S+35 coexist as explicit runtime
//! partitions; interaction focus, floating-origin rebasing, presentation and
//! demand are projections over that stack rather than one privileged scale.

mod demand;
mod devtools;
mod layer;
mod motion;
mod navigation;
mod position;
mod refinement;
mod transition;
mod view;

pub use demand::{
    SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot, SpatialDemandSource,
    SpatialRefinementDemand,
};
pub(crate) use devtools::SPATIAL_DEMAND_VISUALIZATION;
pub use navigation::{
    UsfApproachRefinement, UsfNavigationContext, UsfNavigationContextKind, UsfRadialGravitySource, UsfTravelInfluence,
    UsfTravelInfluenceKind, UsfTravelInfluenceMeasure, UsfTravelMedium,
    UsfTravelNeighborhood,
};
pub use layer::{
    UsfPrimaryInteractionSlice, UsfChartMask, UsfInteractionProjection, UsfScaleLayer,
    UsfScaleLayerFrames, UsfScaleSlice, UsfScaleSliceMemberOf, UsfScaleSliceMembers,
    UsfScaleSlices,
};
pub use motion::UsfCanonicalMotion;
pub use position::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfChunkAddress,
    UsfPosition, UsfPositionError,
};
pub use refinement::{
    UsfRefinementAperture, UsfScaleCoverage, UsfScaleCoverageSnapshot, UsfScaleRoleMask,
};
pub use transition::{
    UsfInteractionRequirement, UsfSpatialTransition, UsfSpatialTransitionApplied,
    UsfSpatialTransitionCause, UsfSpatialTransitionQueue, UsfTransitionVelocity,
};
pub use view::{
    UsfDistanceMeshLod, UsfLocalScalePresentation, UsfScaleFallbackPresentation, UsfScalePresentation,
    UsfSceneryPresentation, UsfViewAnchor, UsfViewContext, UsfViewRenderAnchor,
    UsfViewScaleDemand,
};

use avian3d::prelude::Position;
use bevy::{prelude::*, transform::TransformSystems};

use crate::ecs::{UsfLogicalProjection, UsfManifestationOf};

const REBASE_THRESHOLD_METERS: f32 = 256.0;
const REBASE_QUANTUM_METERS: f32 = 256.0;

/// Marks the logical projection used to anchor the current local runtime chart.
///
/// This is intentionally independent from manifestation authority and from
/// presentation projection. Portal/world-wrap spatial multiplicity may provide
/// other simultaneous logical projections without changing which one anchors
/// this chart.
#[derive(Component, Debug, Default)]
pub struct UsfSpatialAnchor;

/// Current bounded runtime chart over canonical USF space.
#[derive(Resource, Debug, Clone)]
pub struct UsfSpatialFrame {
    origin: UsfPosition,
    rebase_count: u64,
    last_shift: Vec3,
}

impl Default for UsfSpatialFrame {
    fn default() -> Self {
        Self {
            origin: UsfPosition::zero(SpatialScale::MAX),
            rebase_count: 0,
            last_shift: Vec3::ZERO,
        }
    }
}

impl UsfSpatialFrame {
    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }

    pub const fn rebase_count(&self) -> u64 {
        self.rebase_count
    }

    pub const fn last_shift(&self) -> Vec3 {
        self.last_shift
    }
}

/// Emitted after the local chart origin changes. Systems that cache local-space
/// coordinates must translate those caches by the same amount.
#[derive(Message, Debug, Clone, Copy)]
pub struct UsfOriginRebased {
    pub local_shift: Vec3,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfSpatialSet {
    SyncSemantic,
    Rebase,
    ViewAnchor,
    ViewProjection,
}

mod rebase;
mod systems;

use rebase::rebase_local_frame;
use systems::sync_semantic_positions;

pub struct UsfSpatialPlugin;

impl Plugin for UsfSpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfSpatialFrame>()
            .init_resource::<UsfPrimaryInteractionSlice>()
            .init_resource::<UsfScaleLayerFrames>()
            .init_resource::<UsfScaleSlices>()
            .init_resource::<UsfScaleCoverageSnapshot>()
            .init_resource::<UsfSpatialTransitionQueue>()
            .add_message::<UsfOriginRebased>()
            .add_message::<UsfSpatialTransitionApplied>()
            .add_systems(Startup, layer::spawn_scale_slices)
            .add_systems(PreUpdate, refinement::clear_scale_coverage)
            .configure_sets(
                PostUpdate,
                (
                    UsfSpatialSet::SyncSemantic,
                    UsfSpatialSet::Rebase,
                    UsfSpatialSet::ViewAnchor,
                    UsfSpatialSet::ViewProjection,
                )
                    .chain(),
            )
            .configure_sets(
                PostUpdate,
                UsfSpatialSet::ViewProjection.before(TransformSystems::Propagate),
            )
            .add_systems(
                PostUpdate,
                (
                    motion::sync_canonical_motion_from_runtime,
                    sync_semantic_positions,
                    transition::apply_spatial_transitions,
                    layer::sync_scale_slice_membership,
                )
                    .chain()
                    .in_set(UsfSpatialSet::SyncSemantic),
            )
            .add_systems(PostUpdate, rebase_local_frame.in_set(UsfSpatialSet::Rebase))
            .add_systems(
                PostUpdate,
                view::sync_view_context.in_set(UsfSpatialSet::ViewAnchor),
            )
            .add_systems(
                PostUpdate,
                (
                    view::project_local_scale_presentations,
                    view::project_scale_presentations,
                    view::project_scenery_presentations,
                    view::select_distance_mesh_lods,
                )
                    .chain()
                    .in_set(UsfSpatialSet::ViewProjection),
            );

        demand::configure(app);
        devtools::configure(app);
    }
}

#[cfg(test)]
mod tests;
