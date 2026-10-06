//! Live celestial presentation facility over the voxel-local binary resolution domain.
//!
//! This is presentation only. Semantic terrain remains [`CelestialVoxelField`];
//! dense voxel worlds keep collision/editing authority. The clipmap is a
//! reconstructible mesh adapter whose sampling-resolution axis is independent of USF Scale.
//!
//! ## Module map
//!
//! - `lifecycle`: GPU work admission and make-before-break frontier publication.
//! - `model`: Reconstructible clipmap plans, tickets, coverage and telemetry state.
//! - `plan`: Balanced binary frontier planning from semantic field evidence.
//! - `planning`: Planning ticket completion and bounded refresh admission for the live clipmap.
//! - `projection`: View projection and dense presentation fallback.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::{
    camera::{
        primitives::Aabb,
        visibility::{NoAutoAabb, RenderLayers},
    },
    light::{NotShadowCaster, NotShadowReceiver},
    math::DVec3,
    prelude::*,
    render::storage::ShaderBuffer,
};

use crate::procedural_assets::{DEBUG_GRID_BASE_UV_METRES_PER_UNIT, ProceduralPresentationAssets};
use crate::reconstructible::{ReconstructibleFrameBudget, ReconstructibleWorkClass};
use crate::view::USF_PRESENTATION_LAYER;
use crate::voxel::{
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationResidency, VoxelScaleRealization,
};

use crate::{
    ecs::UsfPresentationProjectionOf,
    spatial::{
        SpatialRealizationGranularityRequest, SpatialScale, UsfCapabilitySet, UsfPosition,
        UsfPrimaryInteractionSlice, UsfScaleLayer, UsfScaleRoleMask, UsfSemanticFrame,
        UsfSpatialSet, UsfViewContext, UsfViewDemand, UsfViewDemandSnapshot, UsfViewRenderAnchor,
    },
};

use super::classification::{CelestialClipmapSurfaceCache, ClipmapBoundaryClassifier};
use super::topology::{
    BLOCK_SUBDIVISIONS, CLIPMAP_FACE_DIRECTIONS, CelestialClipmapBlockKey,
    block_contains_local_point, block_distance_squared_to_point, block_distance_to_point,
    leaf_containing_point, same_or_coarser_face_neighbor, transition_faces_for_frontier,
};
use super::visibility::ClipmapVisibilityDemand;

use super::super::{
    CelestialVoxelField, CelestialVoxelRealizationPolicy, CelestialVoxelScaleRealization,
    VoxelSemanticAuthority,
    manifestation::{
        VoxelPresentationFallbackRetireReady, VoxelPresentationGeometry,
        VoxelPresentationManifestation, VoxelRenderMaterial, create_voxel_render_material,
    },
    presentation_palette::{DEBUG_BAND_COUNT, debug_band_rgb},
    worker::{VoxelWorkExecutor, VoxelWorkLane, VoxelWorkTicket},
};
use super::{
    VoxelPresentationResolution, VoxelTransitionFaces,
    gpu::{GpuTerrainBuild, GpuTerrainBuilds, allocation_mesh, descriptor_for_block},
};

mod lifecycle;
mod model;
mod plan;
mod planning;
mod projection;

use lifecycle::*;
use model::*;
pub(in crate::voxel) use model::{
    CelestialClipmapCoverageCell, CelestialClipmapCoverageSnapshot, CelestialClipmapTelemetry,
};
use plan::*;
use projection::*;

// Binary presentation is independent from decimal USF interaction Scale.
const MIN_SAMPLE_SPACING_METRES: f64 = 1.0;
const MAX_FINE_SAMPLE_SPACING_METRES: f64 = 2_048.0;
//
// The coarsest binary bricks are allowed to span the semantic body. A small
// conservative margin absorbs canonical relief without inventing a second
// spherical surface representation.
const WHOLE_BODY_ROOT_MARGIN: f64 = 1.125;
const TARGET_CELLS_PER_DISTANCE: f64 = 16.0;
const TARGET_CELLS_PER_CLEARANCE: f64 = 128.0;
const TARGET_PIXELS_PER_BINARY_SAMPLE: f64 = 4.0;
// Inactive dense presentation is a bootstrap fallback, not a LOD layer or history buffer.
const DENSE_FALLBACK_RETENTION_CHUNKS: f32 = 4.0;

//
// Deep local detail is a sparse boundary aperture over persistent coarse body
// ancestry. The budget scales with requested binary depth; this hard ceiling is
// reconstructible protection, not a normal target.
const MIN_SPARSE_FRONTIER_LEAVES: usize = 4_096;
const MAX_SPARSE_FRONTIER_LEAVES: usize = 32_768;
const LEAVES_PER_REQUESTED_LEVEL: usize = 640;
// Each checkpoint remains a complete balanced Transvoxel frontier, but the
// changed region is deliberately small enough to become visible continuously.
//
// Tiny eight-refinement waves forced repeated balance passes and repeated
// whole-frontier materialization while changing almost nothing. Reconstructible
// planning now does substantial work per local balance transaction and publishes
// only a cold bootstrap plus the final balanced replacement frontier.
const MAX_PRIMARY_REFINEMENTS_PER_WAVE: usize = 256;
//
// Planning remains one deep sparse solve. Publication, however, must not jump
// from eight root bricks directly to a multi-thousand-leaf final transaction.
// Cheap hierarchy-only stage synthesis below exposes balanced intermediate
// frontiers without re-running canonical SDF planning.
const MAX_RECORDED_FRONTIER_STAGES: usize = 8;
const CLIPMAP_VALIDITY_AGGREGATES_ACROSS: u32 = 8;
const CLIPMAP_MIN_VALIDITY_SECONDS: f64 = 0.10;
const CLIPMAP_MAX_VALIDITY_SECONDS: f64 = 2.0;
const CLIPMAP_LATENCY_MULTIPLIER: f64 = 4.0;
// Reconstructible clipmap presentation shells are expensive to churn through
// Bevy's entity/asset lifecycle. Keep a bounded hot pool and mutate stable
// Mesh handles in place, matching the dense manifestation runtime.

#[inline]
pub(super) fn configure(app: &mut App) {
    app.init_resource::<CelestialClipmapRealizations>()
        .init_resource::<CelestialClipmapBandDebugMaterials>()
        .init_resource::<CelestialClipmapCoverageSnapshot>()
        .init_resource::<CelestialTerrainPresentationState>()
        .init_resource::<CelestialClipmapTelemetry>()
        .add_systems(Update, reconcile_celestial_clipmap_realizations)
        .add_systems(
            PostUpdate,
            project_celestial_clipmap_transforms
                .after(UsfCapabilitySet::ReconcileCoverage)
                .in_set(UsfSpatialSet::ViewProjection),
        )
        .add_systems(
            PostUpdate,
            reconcile_dense_presentation_fallback.after(UsfSpatialSet::ViewProjection),
        );
}
