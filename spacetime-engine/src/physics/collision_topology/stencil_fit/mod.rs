//! Rectangular subtractive-stencil fitting against immutable collision sources.
//!
//! ## Module map
//!
//! - `axes`: Face-axis projections shared by fit and strict support checks.
//! - `placement`: Corrects a rectangle onto one cuboid face and its tangent bounds.
//! - `support`: Strict uncorrected support predicate for a rectangular aperture.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

use bevy::prelude::*;

mod axes;
mod placement;
mod support;

use axes::{axis_vector, component, dominant_axis, set_component};
use placement::fit_rectangle_to_cuboid_face;
use support::cuboid_supports_rectangle;

use super::source::CollisionClipSource;

const UNIT_SCALE_EPSILON: f32 = 1.0e-4;
const FACE_ALIGNMENT_DOT: f32 = 0.999;

/// A nearby valid placement for a rectangular subtractive stencil.
#[derive(Debug, Clone, Copy)]
pub struct RectangularStencilFit {
    pub transform: Transform,
    /// World-space correction from the requested center, in metres.
    pub displacement: f32,
    /// Number of tangent axes magnetized/clamped to a support boundary.
    /// Two means the aperture is corner-snapped.
    pub snapped_edges: u8,
}

/// Finds the nearest valid placement of a rectangular stencil on `source`.
///
/// Cuboid faces provide natural exact axes, so the returned rectangle is also
/// roll-quantized to a quarter turn on the selected face. This removes tiny
/// orientation errors while keeping the support/fit policy generic rather than
/// portal-specific.
pub fn fit_rectangular_stencil(
    source: CollisionClipSource,
    host: &Transform,
    requested: &Transform,
    half_size: Vec2,
    plane_tolerance: f32,
    max_translation: f32,
    edge_snap_distance: f32,
) -> Option<RectangularStencilFit> {
    if (host.scale - Vec3::ONE).length_squared() > UNIT_SCALE_EPSILON * UNIT_SCALE_EPSILON {
        return None;
    }

    match source {
        CollisionClipSource::Cuboid { half_extents } => fit_rectangle_to_cuboid_face(
            half_extents,
            host,
            requested,
            half_size,
            plane_tolerance.max(0.0),
            max_translation.max(0.0),
            edge_snap_distance.max(0.0),
        ),
    }
}

/// Returns whether `stencil` lies flush on one face of `source` and its full
/// rectangular aperture fits inside that face.
///
/// This strict predicate is useful when callers do not want correction. Tools
/// that support snapping should use [`fit_rectangular_stencil`] instead.
pub fn supports_rectangular_stencil(
    source: CollisionClipSource,
    host: &Transform,
    stencil: &Transform,
    half_size: Vec2,
    plane_tolerance: f32,
) -> bool {
    if (host.scale - Vec3::ONE).length_squared() > UNIT_SCALE_EPSILON * UNIT_SCALE_EPSILON {
        return false;
    }

    match source {
        CollisionClipSource::Cuboid { half_extents } => cuboid_supports_rectangle(
            half_extents,
            host,
            stencil,
            half_size,
            plane_tolerance.max(0.0),
        ),
    }
}
