//! Generic controlled-subject navigation and travel policy.
//!
//! Subject-owned SI travel state is separate from view-owned presentation
//! state. Runtime systems observe and publish through those contracts.

use crate::spatial::UsfSpatialSet;
use bevy::{app::RunFixedMainLoop, prelude::*};

mod devtools;
mod policy;
mod presentation;
mod runtime;
mod travel;

pub use devtools::NavigationFlightRecorder;
pub use presentation::*;
pub use travel::*;

/// Stable semantic navigation runtime extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavigationSet {
    Observe,
    Plan,
    Publish,
}

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavigationAudit>()
            .init_resource::<NavigationFlightRecorder>()
            .register_type::<ManualTravelProfile>()
            .register_type::<CruiseTravelProfile>()
            .register_type::<PlanetaryTravelProfile>()
            .register_type::<ApproachTravelProfile>()
            .register_type::<FlightDynamicsProfile>()
            .register_type::<TravelProfile>()
            .register_type::<TravelPace>()
            .register_type::<TravelEnvelope>()
            .register_type::<TravelState>()
            .register_type::<AdaptiveCruise>()
            .register_type::<ApproachRefinementState>()
            .register_type::<NavigationPresentationProfile>()
            .register_type::<NavigationPresentationState>()
            .add_systems(
                RunFixedMainLoop,
                (
                    runtime::sync_navigation_context,
                    runtime::sync_travel_state,
                    policy::sync_travel_envelope,
                )
                    .chain()
                    .in_set(NavigationSet::Observe),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    runtime::plan_approach_refinement,
                    runtime::sync_navigation_presentation,
                )
                    .chain()
                    .in_set(NavigationSet::Plan),
            )
            // Navigation may refine ahead and choose presentation/travel policy,
            // but interaction Scale belongs to the controlled manifestation.
            .add_systems(
                RunFixedMainLoop,
                runtime::audit_navigation_contract.in_set(NavigationSet::Publish),
            )
            .add_systems(
                PostUpdate,
                runtime::reconcile_approach_after_requested_transition
                    .after(UsfSpatialSet::SyncSemantic),
            )
            .add_systems(
                PostUpdate,
                devtools::record_navigation_flight.after(UsfSpatialSet::ViewProjection),
            );
    }
}
