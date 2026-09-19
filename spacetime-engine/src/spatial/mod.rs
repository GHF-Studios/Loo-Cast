//! USF semantic spatial identity projected into bounded local runtime coordinates.
//!
//! M7 deliberately implements only fixed-scale (S0) translation and floating
//! origin rebasing. The finite canonical stack wraps on root carry/borrow;
//! scale transitions and observer-relative scale views remain later milestones.

mod demand;
mod devtools;
mod layer;
mod position;
mod view;

pub use demand::{
    SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot, SpatialDemandSource,
};
pub(crate) use devtools::SPATIAL_DEMAND_VISUALIZATION;
pub use layer::{UsfActiveScaleLayer, UsfFollowsActiveScale, UsfScaleLayer, UsfScaleLayerFrames};
pub use position::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfChunkAddress,
    UsfPosition, UsfPositionError,
};
pub use view::{UsfLocalScalePresentation, UsfScalePresentation, UsfViewAnchor, UsfViewFrame};

use avian3d::prelude::{LinearVelocity, Position};
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
use systems::{sync_active_scale_layer, sync_semantic_positions};

pub struct UsfSpatialPlugin;

impl Plugin for UsfSpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfSpatialFrame>()
            .init_resource::<UsfActiveScaleLayer>()
            .init_resource::<UsfScaleLayerFrames>()
            .add_message::<UsfOriginRebased>()
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
                (sync_active_scale_layer, sync_semantic_positions)
                    .chain()
                    .in_set(UsfSpatialSet::SyncSemantic),
            )
            .add_systems(PostUpdate, rebase_local_frame.in_set(UsfSpatialSet::Rebase))
            .add_systems(
                PostUpdate,
                view::sync_view_anchor.in_set(UsfSpatialSet::ViewAnchor),
            )
            .add_systems(
                PostUpdate,
                (
                    view::project_local_scale_presentations,
                    view::project_scale_presentations,
                )
                    .in_set(UsfSpatialSet::ViewProjection),
            );

        demand::configure(app);
        devtools::configure(app);
        view::configure(app);
    }
}

#[cfg(test)]
mod tests;
