//! Observer-relative presentation scale over canonical USF space.
//!
//! Presentation scale is independent from whichever bounded Scale Slice
//! currently owns physical interaction for the controlled subject.
//! A representation authored at scale S stores bounded S-native geometry and a
//! canonical anchor; it never needs a universe-wide float position.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfInteractionProjection, UsfPosition, UsfScaleLayer};

const PRESENTATION_RELATIVE_BOUND: f32 = 16_384.0;
const DIRECT_PRESENTATION_SCALE_BOUND: f64 = PRESENTATION_RELATIVE_BOUND as f64;
const SCENERY_RELATIVE_BOUND: f32 = 1_000_000.0;
const CONTRIBUTION_EPSILON: f32 = 0.001;

mod anchor;
mod context;
mod presentation;
mod probe;

pub use anchor::{UsfViewAnchor, UsfViewObservationOverride, UsfViewRenderAnchor};
pub use context::UsfViewContext;
pub use presentation::{
    UsfLocalScalePresentation, UsfScaleFallbackPresentation, UsfScalePresentation,
    UsfSceneryPresentation,
};
pub use probe::UsfPresentationProbe;

mod demand;
mod lod;
mod systems;

pub use demand::{UsfViewDemand, UsfViewDemandMode, UsfViewDemandPolicy, UsfViewDemandSnapshot};
pub use lod::UsfDistanceMeshLod;

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfViewObservationOverride>();
    demand::configure(app);
}

pub(super) use lod::select_distance_mesh_lods;
pub(super) use systems::{
    project_local_scale_presentations, project_scale_presentations, project_scenery_presentations,
    sync_view_context,
};
