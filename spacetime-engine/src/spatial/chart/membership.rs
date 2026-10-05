//! Scale membership shared by runtime rebase preflight and backend application.

use avian3d::prelude::{ColliderOf, RigidBody};
use bevy::prelude::*;

use crate::spatial::{SpatialScale, UsfScaleLayer};

/// A collider inherits its rigid body's chart when it has no direct layer.
/// Both sides of a rebase must resolve that membership identically.
pub(crate) fn resolved_rebase_scale(
    direct: Option<&UsfScaleLayer>,
    attached: Option<&ColliderOf>,
    body_layers: &Query<&UsfScaleLayer, With<RigidBody>>,
    fallback: SpatialScale,
) -> SpatialScale {
    direct
        .copied()
        .or_else(|| attached.and_then(|a| body_layers.get(a.body).ok().copied()))
        .map_or(fallback, UsfScaleLayer::scale)
}
