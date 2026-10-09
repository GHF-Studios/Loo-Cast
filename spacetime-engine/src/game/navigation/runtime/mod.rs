//! Controlled-subject semantic navigation adapters.
//!
//! ## Module map
//!
//! - `approach`: Controlled-subject refinement intent and transition reconciliation.
//! - `audit`: Read-only navigation and presentation contract diagnostics.
//! - `context`: Sparse navigation neighborhood and characteristic scale observation.
//! - `presentation`: View-owned presentation policy downstream of navigation context.
//! - `travel`: Subject-owned travel state and primary body selection.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    game::control::LocalControlSubject,
    spatial::{
        SpatialRefinementDemand, SpatialScale, UsfApproachRefinement, UsfCanonicalMotion,
        UsfNavigationContext, UsfPosition, UsfScaleLayer, UsfSemanticFrame,
        UsfSpatialTransitionApplied, UsfSpatialTransitionCause, UsfTravelBoundaryProvider,
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
pub(super) use travel::{resolve_travel_assistance, sync_travel_state};
