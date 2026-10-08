//! Observer-relative presentation scale over canonical USF space.
//!
//! Presentation scale is independent from whichever bounded Scale Slice
//! currently owns physical interaction for the controlled subject.
//! A representation authored at scale S stores bounded S-native geometry and a
//! canonical anchor; it never needs a universe-wide float position.
//!
//! ## Module map
//!
//! - `anchor`: Semantic and render view anchors plus observer-only override.
//! - `context`: View-owned observer state and bounded projection conversions.
//! - `presentation`: Scale-local presentation contracts.
//! - `probe`: Diagnostic presentation filter; never semantic authority.
//! - `demand`: Observer-derived sparse presentation demand over the USF Scale Stack.
//! - `systems`: ECS realization of observer-relative USF presentation state.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfInteractionProjection, UsfPosition, UsfScaleLayer};

const PRESENTATION_RELATIVE_BOUND: f32 = 16_384.0;
const DIRECT_PRESENTATION_SCALE_BOUND: f64 = PRESENTATION_RELATIVE_BOUND as f64;
const CONTRIBUTION_EPSILON: f32 = 0.001;

mod anchor;
mod context;
mod presentation;
mod probe;

pub use anchor::{UsfViewAnchor, UsfViewObservationOverride, UsfViewRenderAnchor};
pub use context::UsfViewContext;
pub use presentation::{UsfLocalScalePresentation, UsfScalePresentation};
pub use probe::UsfPresentationDomainProbe;

mod demand;
mod systems;

pub use demand::{UsfViewDemand, UsfViewDemandMode, UsfViewDemandPolicy, UsfViewDemandSnapshot};

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfViewObservationOverride>();
    demand::configure(app);
}

pub(super) use systems::{
    project_local_scale_presentations, project_scale_presentations, sync_view_context,
};
