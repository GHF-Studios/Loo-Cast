//! USF semantic spatial identity projected into bounded local runtime coordinates.
//!
//! Canonical positions remain independent from bounded local runtime charts.
//! All 71 Scale Slices from S-35 through S+35 coexist as explicit runtime
//! partitions; interaction focus, floating-origin rebasing, presentation and
//! demand are projections over that stack rather than one privileged scale.
//!
//! ## Integration
//!
//! USF owns canonical number algebra; this module owns runtime projection, demand, coverage,
//! refinement, residency, transitions, and views over that algebra. Capability coverage reports
//! realized state rather than desired state.
//!
//! ## Module map
//!
//! - `capability`: Generic scale-local capability realization and realized coverage.
//! - `demand`: Generic bounded spatial interest.
//! - `devtools`: Install spatial inspection panels and World Draw diagnostics.
//! - `interaction`: Controlled-subject interaction focus over the persistent Scale Slice stack.
//! - `kinematic_frame`: Time-dependent canonical kinematic frames.
//! - `motion`: Canonical physical motion projected into bounded Scale-Slice runtime charts.
//! - `navigation`: Semantic long-distance navigation.
//! - `realization`: Capability-local realization granularity and temporal-validity policy.
//! - `refinement`: Current-relative multiscale refinement planning.
//! - `residency`: Ancestor-closed runtime residency over canonical USF space.
//! - `semantic_frame`: Canonical orientation for movable semantic spatial frames.
//! - `slice`: The 71 explicit USF Scale Slices and scale-local runtime membership.
//! - `transition`: First-class canonical spatial transitions.
//! - `view`: Observer-relative presentation scale over canonical USF space.
//! - `chart`: Engine-owned maintenance of the current bounded runtime chart.
//! - `systems`: ECS synchronization between bounded runtime projections and canonical USF state.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod capability;
mod demand;
mod devtools;
mod interaction;
mod kinematic_frame;
mod motion;
mod navigation;
mod realization;
mod refinement;
mod residency;
mod semantic_frame;
mod slice;
mod transition;
mod view;

pub use crate::usf::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfChart,
    UsfChartDelta, UsfChunkAddress, UsfPosition, UsfPositionError,
};
pub use capability::{
    UsfCapabilityCoverageFact, UsfCapabilityCoveragePublication, UsfCapabilityRealization,
    UsfCapabilitySet, UsfRefinementAperture, UsfScaleCoverage, UsfScaleCoverageSnapshot,
    UsfScaleRoleMask,
};
pub(crate) use chart::resolved_rebase_scale;
pub use demand::{
    SpatialDemandMotionSnapshot, SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot,
    SpatialDemandSource,
};
pub(crate) use devtools::SPATIAL_DEMAND_VISUALIZATION;
pub use interaction::{
    UsfInteractionProjection, UsfInteractionScaleAffinity, UsfPrimaryInteractionSlice,
};
pub use kinematic_frame::UsfKinematicFrameState;
pub use motion::{UsfCanonicalMotion, UsfMotionAuthority};
pub use navigation::{
    UsfApproachRefinement, UsfNavigationContext, UsfNavigationContextKind, UsfTravelBoundary,
    UsfTravelBoundaryProvider, UsfTravelBoundarySample, UsfTravelInfluence, UsfTravelInfluenceKind,
    UsfTravelInfluenceMeasure, UsfTravelMedium, UsfTravelNeighborhood,
};
pub use realization::{SpatialRealizationGranularity, SpatialRealizationGranularityRequest};
pub use refinement::{SpatialRefinementDemand, UsfRefinementPlan, UsfRefinementStep};
pub use residency::{
    UsfContextResidency, UsfResidencyRequests, UsfResidencySet, UsfResidentContext,
};
pub use semantic_frame::UsfSemanticFrame;
pub use slice::{
    UsfScaleLayer, UsfScaleSlice, UsfScaleSliceMask, UsfScaleSliceMemberOf, UsfScaleSliceMembers,
    UsfScaleSlices,
};
pub use transition::{
    UsfInteractionHandoffGuards, UsfInteractionRequirement, UsfSpatialTransition,
    UsfSpatialTransitionApplied, UsfSpatialTransitionCause, UsfSpatialTransitions,
    UsfTransitionVelocity,
};
pub use view::{
    UsfLocalScalePresentation, UsfPresentationDomainProbe, UsfScalePresentation, UsfViewAnchor,
    UsfViewContext, UsfViewDemand, UsfViewDemandMode, UsfViewDemandPolicy, UsfViewDemandSnapshot,
    UsfViewObservationOverride, UsfViewRenderAnchor,
};

use bevy::{prelude::*, transform::TransformSystems};

pub use chart::{UsfOriginRebased, UsfRuntimeChartState, UsfSpatialAnchor};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfInteractionHandoffSet {
    Reset,
    Providers,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfSpatialSet {
    SyncSemantic,
    Rebase,
    /// Rebuild runtime-local projections/caches from the new canonical frame.
    RuntimeProjection,
    /// Make backend acceleration structures observe the rebuilt projections.
    BackendRefresh,
    ViewAnchor,
    ViewProjection,
}

mod chart;
mod systems;

use chart::rebase_local_frame;
use systems::sync_semantic_positions;

pub struct UsfSpatialPlugin;

impl Plugin for UsfSpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfRuntimeChartState>()
            .init_resource::<UsfPrimaryInteractionSlice>()
            .init_resource::<UsfInteractionHandoffGuards>()
            .init_resource::<UsfPresentationDomainProbe>()
            .init_resource::<UsfScaleSlices>()
            .init_resource::<UsfSpatialTransitions>()
            .add_message::<UsfOriginRebased>()
            .add_message::<UsfSpatialTransitionApplied>()
            .add_systems(Startup, slice::spawn_scale_slices)
            .add_systems(
                PostUpdate,
                transition::reset_interaction_handoff_guards
                    .in_set(UsfInteractionHandoffSet::Reset),
            )
            // Spatial synchronization is a dependency DAG, not one global
            // serialized pipeline. Runtime projection and view anchoring both
            // need the rebased canonical frame, but view work does not depend
            // on physics-backend acceleration refresh. Keeping these edges
            // explicit lets Bevy overlap independent backend/view work while
            // preserving every semantic freshness requirement.
            .configure_sets(
                PostUpdate,
                (
                    UsfInteractionHandoffSet::Reset.after(UsfCapabilitySet::ReconcileCoverage),
                    UsfInteractionHandoffSet::Providers
                        .after(UsfInteractionHandoffSet::Reset)
                        .before(UsfSpatialSet::SyncSemantic),
                    UsfSpatialSet::SyncSemantic.after(UsfInteractionHandoffSet::Providers),
                    UsfSpatialSet::Rebase.after(UsfSpatialSet::SyncSemantic),
                    UsfSpatialSet::RuntimeProjection.after(UsfSpatialSet::Rebase),
                    UsfSpatialSet::BackendRefresh.after(UsfSpatialSet::RuntimeProjection),
                    UsfSpatialSet::ViewAnchor.after(UsfSpatialSet::Rebase),
                    UsfSpatialSet::ViewProjection
                        .after(UsfSpatialSet::RuntimeProjection)
                        .after(UsfSpatialSet::ViewAnchor),
                ),
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
                    slice::sync_scale_slice_membership,
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
                )
                    .chain()
                    .in_set(UsfSpatialSet::ViewProjection),
            );

        capability::configure(app);
        demand::configure(app);
        residency::configure(app);
        view::configure(app);
        devtools::configure(app);
    }
}
