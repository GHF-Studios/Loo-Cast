//! Controlled-subject semantic navigation adapters.

use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    game::control::LocalControlSubject,
    spatial::{
        SpatialRefinementDemand, SpatialScale, UsfApproachRefinement, UsfNavigationContext,
        UsfPosition, UsfRuntimeChartState, UsfScaleLayer, UsfSemanticFrame,
        UsfSpatialTransitionApplied, UsfSpatialTransitionCause, UsfTravelBoundaryResolver,
        UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelNeighborhood, UsfViewContext,
        UsfViewRenderAnchor,
    },
};

use super::{
    ApproachRefinementState, NavigationAudit, NavigationPresentationProfile,
    NavigationPresentationState, PrimaryBodyContext, TravelEnvelope, TravelProfile, TravelState,
};

mod approach;
mod audit;
mod context;
mod presentation;
mod travel;

pub(super) use approach::{
    plan_approach_refinement, reconcile_approach_after_requested_transition,
};
pub(super) use audit::audit_navigation_contract;
pub(super) use context::sync_navigation_context;
pub(super) use presentation::sync_navigation_presentation;
pub(super) use travel::sync_travel_state;
